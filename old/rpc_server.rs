use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tonic::{transport::Server as TonicServer, Request, Response, Status};
use crate::protos::{
    validator_service_server::{ValidatorService, ValidatorServiceServer},
    Empty, GetHistoryQuery, GetHistoryResponse, GetLatestQuery, GetLatestResponse,
    ProposeMessage, VoteMessage, TxGossip, HelloMessage,
};

pub struct ValidatorServiceImpl {
    sender: mpsc::Sender<ProposeMessage>,
    receiver: Arc<Mutex<mpsc::Receiver<ProposeMessage>>>,
}

#[tonic::async_trait]
impl ValidatorService for ValidatorServiceImpl {
    async fn get_history(
        &self,
        request: Request<GetHistoryQuery>,
    ) -> Result<Response<GetHistoryResponse>, Status> {
        let query = request.into_inner();
        // TODO: Implement history retrieval from backing store
        Ok(Response::new(GetHistoryResponse {
            blocks: vec![],
            votes: vec![],
        }))
    }

    async fn get_latest(
        &self,
        _request: Request<GetLatestQuery>,
    ) -> Result<Response<GetLatestResponse>, Status> {
        // TODO: Implement latest block retrieval
        Ok(Response::new(GetLatestResponse {
            votes: vec![],
            local_time: chrono::Utc::now().timestamp_millis(),
        }))
    }

    type GossipTxsStream = tokio_stream::wrappers::ReceiverStream<Result<TxGossip, Status>>;

    async fn gossip_txs(
        &self,
        request: Request<tonic::Streaming<TxGossip>>,
    ) -> Result<Response<Self::GossipTxsStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(tx_gossip) = stream.message().await.unwrap() {
                // Process incoming transactions
                // TODO: Implement transaction processing
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    async fn propose(
        &self,
        request: Request<ProposeMessage>,
    ) -> Result<Response<Empty>, Status> {
        let propose_msg = request.into_inner();
        self.sender.send(propose_msg).await.map_err(|e| {
            Status::internal(format!("Failed to process propose message: {}", e))
        })?;
        Ok(Response::new(Empty {}))
    }

    async fn prevote(
        &self,
        request: Request<VoteMessage>,
    ) -> Result<Response<Empty>, Status> {
        // TODO: Implement prevote handling
        Ok(Response::new(Empty {}))
    }

    async fn precommit(
        &self,
        request: Request<VoteMessage>,
    ) -> Result<Response<Empty>, Status> {
        // TODO: Implement precommit handling
        Ok(Response::new(Empty {}))
    }

    async fn hello(
        &self,
        request: Request<HelloMessage>,
    ) -> Result<Response<Empty>, Status> {
        Ok(Response::new(Empty {}))
    }
}

pub struct Server {
    pub addr: SocketAddr,
    sender: mpsc::Sender<ProposeMessage>,
    receiver: Arc<Mutex<mpsc::Receiver<ProposeMessage>>>,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        let (sender, receiver) = mpsc::channel(100);
        Server {
            addr,
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn get_receiver(&self) -> Arc<Mutex<mpsc::Receiver<ProposeMessage>>> {
        self.receiver.clone()
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.addr;
        println!("gRPC Server running on {}", addr);

        let service = ValidatorServiceImpl {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        };

        TonicServer::builder()
            .add_service(ValidatorServiceServer::new(service))
            .serve(addr)
            .await?;

        Ok(())
    }
}

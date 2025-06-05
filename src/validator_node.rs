use tonic::{transport::Server, Request, Response, Status};
use crate::protos::validator_service_server::{ValidatorService, ValidatorServiceServer};
use crate::protos::{ProposeMessage, VoteMessage, GetHistoryQuery, GetHistoryResponse, GetLatestQuery, GetLatestResponse, HelloMessage, Empty, TxGossip};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use std::pin::Pin;
use tokio_stream::Stream;

pub struct ValidatorNode {}

#[tonic::async_trait]
impl ValidatorService for ValidatorNode {
    type GossipTxsStream = Pin<Box<dyn Stream<Item = Result<TxGossip, Status>> + Send + 'static>>;
    
    async fn get_history(&self, request: Request<GetHistoryQuery>) -> Result<Response<GetHistoryResponse>, Status> {
        // Implement the get_history logic here
        let response = GetHistoryResponse {
            decisions: vec![],
        };
        Ok(Response::new(response))
    }

    async fn get_latest(&self, request: Request<GetLatestQuery>) -> Result<Response<GetLatestResponse>, Status> {
        // Implement the get_latest logic here
        let response = GetLatestResponse {
            votes: vec![],
            local_time: 0,
        };
        Ok(Response::new(response))
    }

    async fn hello(&self, request: Request<HelloMessage>) -> Result<Response<Empty>, Status> {
        // Implement the hello logic here
        let response = Empty {};
        Ok(Response::new(response))
    }

    async fn gossip_txs(&self, _request: Request<tonic::Streaming<TxGossip>>) -> Result<Response<Self::GossipTxsStream>, Status> {
        // Provide an empty stream for now
        let (_tx, rx) = mpsc::channel(4);
        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::GossipTxsStream))
    }

    async fn propose(&self, request: Request<ProposeMessage>) -> Result<Response<Empty>, Status> {
        // Implement the propose logic here
        let response = Empty {};
        Ok(Response::new(response))
    }

    async fn prevote(&self, request: Request<VoteMessage>) -> Result<Response<Empty>, Status> {
        // Implement the prevote logic here
        let response = Empty {};
        Ok(Response::new(response))
    }

    async fn precommit(&self, request: Request<VoteMessage>) -> Result<Response<Empty>, Status> {
        // Implement the precommit logic here
        let response = Empty {};
        Ok(Response::new(response))
    }
}

impl ValidatorNode {
    pub async fn run_service(self, addr: String) -> Result<(), Box<dyn std::error::Error>> {
        let addr = addr.parse()?;
        let validator_service = ValidatorServiceServer::new(self);
        Server::builder().add_service(validator_service).serve(addr).await?;
        Ok(())
    }
}


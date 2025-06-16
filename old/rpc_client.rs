use std::net::SocketAddr;
use tonic::transport::Channel;
use crate::protos::{
    validator_service_client::ValidatorServiceClient,
    GetHistoryQuery, GetHistoryResponse, GetLatestQuery, GetLatestResponse,
    ProposeMessage, VoteMessage, TxGossip, Transaction,
};

pub struct Client {
    client: ValidatorServiceClient<Channel>,
}

impl Client {
    pub async fn new(addr: SocketAddr) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ValidatorServiceClient::connect(format!("http://{}", addr)).await?;
        Ok(Client { client })
    }

    pub async fn get_history(
        &mut self,
        start_height: i64,
        end_height: i64,
    ) -> Result<GetHistoryResponse, Box<dyn std::error::Error>> {
        let request = GetHistoryQuery {
            start_height,
            end_height,
        };
        let response = self.client.get_history(request).await?;
        Ok(response.into_inner())
    }

    pub async fn get_latest(&mut self) -> Result<GetLatestResponse, Box<dyn std::error::Error>> {
        let request = GetLatestQuery {};
        let response = self.client.get_latest(request).await?;
        Ok(response.into_inner())
    }

    pub async fn propose(
        &mut self,
        propose_msg: ProposeMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client.propose(propose_msg).await?;
        Ok(())
    }

    pub async fn prevote(
        &mut self,
        vote_msg: VoteMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client.prevote(vote_msg).await?;
        Ok(())
    }

    pub async fn precommit(
        &mut self,
        vote_msg: VoteMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.client.precommit(vote_msg).await?;
        Ok(())
    }

    pub async fn gossip_txs(
        &mut self,
        txs: Vec<Transaction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(futures::stream::iter(vec![TxGossip { txs }]));
        let mut stream = self.client.gossip_txs(request).await?.into_inner();
        
        while let Some(response) = stream.message().await? {
            // Process incoming transaction gossip
            // TODO: Implement transaction gossip handling
        }
        
        Ok(())
    }
}

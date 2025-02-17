#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::thread::{spawn, sleep, JoinHandle};
use reqwest::blocking::ClientBuilder;

static URL: &str = "http://robocraftstaticdata.s3.amazonaws.com/live/data.json";

#[derive(Serialize, Deserialize)]
struct StaticDataRaw {
    MaintenanceMode: String, // bool as string
    MaintenanceRegex: String, // ???
    EacEnabled: String, // bool as string
    MinimumVersion: String, // integer as string
    PhotonSocialServer: String, // url
    PhotonServicesServer: String, // url
    PhotonChatServer: String, // url
    PhotonSinglePlayerServer: String, // url
    GameplayServerServiceAddress: String, // url
    PhotonLobbyServer: String, // url
    ErrorLogAddress: String, // url
    ServerErrorLogAddress: String, // url
    authUrl: String, // url
    paymentUrl: String, // url
    enterBattleLogGenerationTimeout: String, // int as string
    GameServerConnectionTestTimeout: usize,
    AvatarCdnUrl: String, // url
    ClanAvatarCdnUrl: String, // url
    FeatureThrottlerOnPercent: String, // int as string
    EmailCaptureEnabled: String, // bool as string
    UnreliableMessages: String, // bool as string
    MessageQueueEnabled: String, // bool as string
    BrawlDataUrl: String, // url
    CampaignDataUrl: String, // url
    LeaderboardsUrl: String, // url
    NetworkChannelTypes: String, // int as string
    MaxSentMessageQueueSize: usize,
    IsAcksLong: usize, // bool as int (1 === True)
    NetworkDropThreshold: usize,
    PacketSize: usize,
    MaxPacketSize: usize,
    MaxCombinedReliableMessageCount: usize,
    MaxCombinedReliableMessageSize: usize,
    SaveRequestOnPhoton: String, // bool as string
    UseS3System: String, // bool as string
    authMigrationUrl: String, // url
    xsollaEnabled: String, // bool as string
    MaintenanceMessage: String,
    DevMessage: String,
    DevMessageDisplayTime: String, // int as string
}

impl StaticDataRaw {
    pub fn nice(&self) -> StaticData {
        StaticData{
            MaintenanceMode: self.MaintenanceMode == "true",
            MaintenanceRegex: self.MaintenanceRegex.clone(),
            EacEnabled: self.EacEnabled == "true",
            MinimumVersion: self.MinimumVersion.parse::<usize>().unwrap(),
            PhotonSocialServer: self.PhotonSocialServer.clone(),
            PhotonServicesServer: self.PhotonServicesServer.clone(),
            PhotonChatServer: self.PhotonChatServer.clone(),
            PhotonSinglePlayerServer: self.PhotonSinglePlayerServer.clone(),
            GameplayServerServiceAddress: self.GameplayServerServiceAddress.clone(),
            PhotonLobbyServer: self.PhotonLobbyServer.clone(),
            ErrorLogAddress: self.ErrorLogAddress.clone(),
            ServerErrorLogAddress: self.ServerErrorLogAddress.clone(),
            authUrl: self.authUrl.clone(),
            paymentUrl: self.paymentUrl.clone(),
            enterBattleLogGenerationTimeout: self.enterBattleLogGenerationTimeout.parse::<usize>().unwrap(),
            GameServerConnectionTestTimeout: self.GameServerConnectionTestTimeout,
            AvatarCdnUrl: self.AvatarCdnUrl.clone(),
            ClanAvatarCdnUrl: self.ClanAvatarCdnUrl.clone(),
            FeatureThrottlerOnPercent: self.FeatureThrottlerOnPercent.parse::<usize>().unwrap(),
            EmailCaptureEnabled: self.EmailCaptureEnabled.clone(),
            UnreliableMessages: self.UnreliableMessages.clone(),
            MessageQueueEnabled: self.MessageQueueEnabled.clone(),
            BrawlDataUrl: self.BrawlDataUrl.clone(),
            CampaignDataUrl: self.CampaignDataUrl.clone(),
            LeaderboardsUrl: self.LeaderboardsUrl.clone(),
            NetworkChannelTypes: self.NetworkChannelTypes.parse::<usize>().unwrap(),
            MaxSentMessageQueueSize: self.MaxSentMessageQueueSize,
            IsAcksLong: self.IsAcksLong == 1,
            NetworkDropThreshold: self.NetworkDropThreshold,
            PacketSize: self.PacketSize,
            MaxPacketSize: self.MaxPacketSize,
            MaxCombinedReliableMessageCount: self.MaxCombinedReliableMessageCount,
            MaxCombinedReliableMessageSize: self.MaxCombinedReliableMessageSize,
            SaveRequestOnPhoton: self.SaveRequestOnPhoton == "true",
            UseS3System: self.UseS3System == "true",
            authMigrationUrl: self.authMigrationUrl.clone(),
            xsollaEnabled: self.xsollaEnabled == "true",
            MaintenanceMessage: self.MaintenanceMessage.clone(),
            DevMessage: self.DevMessage.clone(),
            DevMessageDisplayTime: self.DevMessageDisplayTime.parse::<usize>().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StaticData {
    pub MaintenanceMode: bool, // bool as string
    pub MaintenanceRegex: String, // ???
    pub EacEnabled: bool, // bool as string
    pub MinimumVersion: usize, // integer as string
    pub PhotonSocialServer: String, // url
    pub PhotonServicesServer: String, // url
    pub PhotonChatServer: String, // url
    pub PhotonSinglePlayerServer: String, // url
    pub GameplayServerServiceAddress: String, // url
    pub PhotonLobbyServer: String, // url
    pub ErrorLogAddress: String, // url
    pub ServerErrorLogAddress: String, // url
    pub authUrl: String, // url
    pub paymentUrl: String, // url
    pub enterBattleLogGenerationTimeout: usize, // int as string
    pub GameServerConnectionTestTimeout: usize,
    pub AvatarCdnUrl: String, // url
    pub ClanAvatarCdnUrl: String, // url
    pub FeatureThrottlerOnPercent: usize, // int as string
    pub EmailCaptureEnabled: String, // bool as string
    pub UnreliableMessages: String, // bool as string
    pub MessageQueueEnabled: String, // bool as string
    pub BrawlDataUrl: String, // url
    pub CampaignDataUrl: String, // url
    pub LeaderboardsUrl: String, // url
    pub NetworkChannelTypes: usize, // int as string
    pub MaxSentMessageQueueSize: usize,
    pub IsAcksLong: bool, // bool as int (1 === True)
    pub NetworkDropThreshold: usize,
    pub PacketSize: usize,
    pub MaxPacketSize: usize,
    pub MaxCombinedReliableMessageCount: usize,
    pub MaxCombinedReliableMessageSize: usize,
    pub SaveRequestOnPhoton: bool, // bool as string
    pub UseS3System: bool, // bool as string
    pub authMigrationUrl: String, // url
    pub xsollaEnabled: bool, // bool as string
    pub MaintenanceMessage: String,
    pub DevMessage: String,
    pub DevMessageDisplayTime: usize, // int as string
}

impl StaticData {
    pub fn new() -> StaticData {
        StaticData{
            MaintenanceMode: false,
            MaintenanceRegex: "".to_string(),
            EacEnabled: false,
            MinimumVersion: 0,
            PhotonSocialServer: "".to_string(),
            PhotonServicesServer: "".to_string(),
            PhotonChatServer: "".to_string(),
            PhotonSinglePlayerServer: "".to_string(),
            GameplayServerServiceAddress: "".to_string(),
            PhotonLobbyServer: "".to_string(),
            ErrorLogAddress: "".to_string(),
            ServerErrorLogAddress: "".to_string(),
            authUrl: "".to_string(),
            paymentUrl: "".to_string(),
            enterBattleLogGenerationTimeout: 0,
            GameServerConnectionTestTimeout: 0,
            AvatarCdnUrl: "".to_string(),
            ClanAvatarCdnUrl: "".to_string(),
            FeatureThrottlerOnPercent: 0,
            EmailCaptureEnabled: "".to_string(),
            UnreliableMessages: "".to_string(),
            MessageQueueEnabled: "".to_string(),
            BrawlDataUrl: "".to_string(),
            CampaignDataUrl: "".to_string(),
            LeaderboardsUrl: "".to_string(),
            NetworkChannelTypes: 0,
            MaxSentMessageQueueSize: 0,
            IsAcksLong: false,
            NetworkDropThreshold: 0,
            PacketSize: 0,
            MaxPacketSize: 0,
            MaxCombinedReliableMessageCount: 0,
            MaxCombinedReliableMessageSize: 0,
            SaveRequestOnPhoton: false,
            UseS3System: false,
            authMigrationUrl: "".to_string(),
            xsollaEnabled: false,
            MaintenanceMessage: "".to_string(),
            DevMessage: "".to_string(),
            DevMessageDisplayTime: 0
        }
    }
}

pub fn start_worker() -> JoinHandle<()> {
    spawn(staticdata_worker)
}

fn staticdata_worker() {
    let mut sleep_dur = std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms);
    let http_client = ClientBuilder::new()
        .connect_timeout(sleep_dur)
        .timeout(sleep_dur)
        .build().expect("Failed to build static data worker HTTP client");
    while ! *crate::IS_STOPPING.read().unwrap() {
        println!("Running static data worker");
        // do work
        let result = http_client.get(URL).send();
        if let Ok(resp) = result {
            //let ctx_old = crate::CONTEXT.read().unwrap().clone();
            let data_res = resp.json::<StaticDataRaw>();
            if let Ok(data) = data_res {
                // json is good
                let nice_data = data.nice();
                let mut ctx = crate::CONTEXT.write().unwrap();
                ctx.staticdata_ok = true;
                ctx.staticdata = nice_data;
                println!("Updated static data successfully");
            } else {
                // json is bad
                let mut ctx = crate::CONTEXT.write().unwrap();
                ctx.staticdata_ok = false;
                //println!("Json err: {}", data_res.err().unwrap());
                println!("Updated static data unsuccessfully (bad JSON)");
            }
        } else {
            // bad server response
            let mut ctx = crate::CONTEXT.write().unwrap();
            ctx.staticdata_ok = false;
            //println!("HTTP error: {}", result.err().unwrap());
            println!("Updated static data unsuccessfully (bad response)");
        }
        // no API spam
        sleep_dur = std::time::Duration::from_millis(crate::CONFIG.read().unwrap().period_ms);
        sleep(sleep_dur);
    }
}




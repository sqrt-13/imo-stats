use std::{fmt::{self, format}, time::Duration};

use reqwest::{blocking::{Client}, StatusCode};

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    HttpRequest(String),
    IoError(String  ),
    UnknownAvailabilityZone(String),
    JsonError(String),
    NotFound(&'static str), // Reported for static URIs we fetch.
}

pub enum IPVersion {
    Ipv4,
    Ipv6,
}

pub struct IMDSClient {
    client: Client,
    base_url: String,
    token_ttl: u64,
    token: Option<String>,
    api_version: String,
}

impl IMDSClient {
    fn set_token(&mut self) -> Result<()> {
        const TOKEN_TTL_HEADER: &str = "X-aws-ec2-metadata-token-ttl-seconds";

        const TOKEN_API_URL: &str = "api/token";
        let resp = self
            .client
            .put(format!("{}/{}", self.base_url, TOKEN_API_URL))
            .header(TOKEN_TTL_HEADER, self.token_ttl)
            .send();

        let resp = resp.map_err(|err| {
            Error::HttpRequest(format!("Error while retrieving token: {}", err.to_string()))
        })?;

        let token = resp.text().map_err(|err| {
            Error::IoError(format!("Error unwrapping response text: {}", err.to_string()))
        })?;

        self.token = Some(token);

        Ok(())
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}/{}", &self.base_url, path)
    }

    pub fn send_command(&mut self, command: IMDSCommand) -> std::result::Result<String, Error> {
        const METADATA_TOKEN_HEADER: &str = "X-aws-ec2-metadata-token";
        let cmd = command.clone();

        let resp = self.client.get(self.build_url(cmd.into())).header(METADATA_TOKEN_HEADER, self.token.as_ref().unwrap()).send()?;

        if resp.status() == StatusCode::UNAUTHORIZED {
            self.set_token();
            return self.send_command(command);
        }

        if resp.status() == StatusCode::NOT_FOUND {
            return Err(Error::NotFound(command.into()));
        }

        resp.text().into()
    }


    pub fn new(ip_v: IPVersion, token_ttl: Option<u64>, api_v: Option<&str>) -> Self {
        let default_token_ttl: u64 = 21600;
        let client = Client::builder().timeout(Duration::from_millis(2000)).build().expect("cannot build http client");
        let token_ttl = token_ttl.unwrap_or(default_token_ttl);
        let api_v = api_v.unwrap_or("latest");
        let mut client = IMDSClient {
            token: None,
            client: client,
            api_version: api_v.into(),
            base_url: format!("http://{}", ip_v),
            token_ttl: token_ttl,
        };

        client.set_token();

        client
    }
}

impl fmt::Display for IPVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IPVersion::Ipv4 => write!(f, "169.254.169.254"),
            IPVersion::Ipv6 => write!(f, "[fd00:ec2::254]"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::HttpRequest(format!("{:?}", error))
    }
}

#[derive(Clone)]
pub enum IMDSCommand {
    ListOperations,
    ListVersions,
    ApiToken,
    /// The AMI ID used to launch the instance.
    AmiId, 
    /// If you started more than one instance at the same time, this value indicates the order in which the instance was launched. The value of the first instance launched is 0.
    AmiLaunchIndex, 
    /// The path to the AMI manifest file in Amazon S3. If you used an Amazon EBS-backed AMI to launch the instance, the returned result is unknown.
    AmiManifestPath,
    /// The ID of this instance.
    InstanceId, 
    // The type of instance
    InstanceType,
    /// In cases where multiple network interfaces are present, this refers to the eth0 device (the device for which the device number is 0). If the EC2 instance is using IP-based naming (IPBN), this is the private IPv4 DNS hostname of the instance. If the EC2 instance is using Resource-based naming (RBN), this is the RBN. For more information about IPBN, RBN, and EC2 instance naming, see Amazon EC2 instance hostname types.
    LocalHostname,
    /// The instance's public DNS (IPv4). This category is only returned if the enableDnsHostnames attribute is set to true. For more information, see DNS attributes for your VPC in the Amazon VPC User Guide. If the instance only has a public-IPv6 address and no public-IPv4 address, this item is not set and results in an HTTP 404 response.
    PublicHostname,
    /// The virtual device that contains the root/boot file system.	
    BlockDeviceMappingAmi,
    /// If the EC2 instance is using IP-based naming (IPBN), this is the private IPv4 DNS hostname of the instance. If the EC2 instance is using Resource-based naming (RBN), this is the RBN. In cases where multiple network interfaces are present, this refers to the eth0 device (the device for which the device number is 0). For more information about IPBN and RBN, see Amazon EC2 instance hostname types.
    Hostname,

    LocalIpv4,
    Profile,
    /// The public IPv4 address. If an Elastic IP address is associated with the instance, the value returned is the Elastic IP address.	
    PublicIpv4,
    PublicKeys,
    /// The ID of the reservation.	
    ReservationId,
}

impl Into<&'static str> for IMDSCommand {
    fn into(self) -> &'static str {
            match self {
            // List Operations
            IMDSCommand::ListOperations => "meta-data",
            IMDSCommand::ListVersions =>  "",
            // Ami Operations
            IMDSCommand::AmiId =>  "meta-data/ami-id",
            IMDSCommand::AmiLaunchIndex =>  "meta-data/ami-launch-index",
            IMDSCommand::AmiManifestPath =>  "meta-data/ami-manifest-path",
            // Block Device Mapping
            IMDSCommand::BlockDeviceMappingAmi =>  "meta-data/block-device-mapping/ami",
            IMDSCommand::Hostname =>  "meta-data/hostname",
            IMDSCommand::InstanceId =>  "meta-data/instance-id",
            IMDSCommand::InstanceType =>  "meta-data/instance-type",
            IMDSCommand::LocalHostname =>  "meta-data/local-hostname",
            IMDSCommand::PublicHostname =>  "meta-data/public-hostname",
            IMDSCommand::ApiToken =>  "api/token",
            IMDSCommand::LocalIpv4 =>  "meta-data/public-hostname",
            IMDSCommand::Profile =>  "meta-data/profile",
            IMDSCommand::PublicIpv4 =>  "meta-data/public-ipv4",
            IMDSCommand::PublicKeys =>  "meta-data/public-hostname",
            IMDSCommand::ReservationId =>  "meta-data/public-hostname",
        }
    }


}


#[cfg(test)]
mod tests {
    use crate::IMDSClient;
    use crate::IPVersion;

    #[test]
    fn inits_ipv4_client_correctly() {
        let client = IMDSClient::new(IPVersion::Ipv4, None, None);
        
        assert_eq!(client.token_ttl, 21600);
        assert_eq!(client.base_url, "http://169.254.169.254");
        assert_eq!(client.api_version, "latest");
    }

    #[test]
    fn inits_ipv6_client_correctly() {
        let client = IMDSClient::new(IPVersion::Ipv6, None, None);
        
        assert_eq!(client.token_ttl, 21600);
        assert_eq!(client.base_url, "http://[fd00:ec2::254]");
        assert_eq!(client.api_version, "latest");
    }

    #[test]
    fn inits_client_with_correct_params() {
        let client = IMDSClient::new(IPVersion::Ipv4, Some(30), Some("2020-01-01"));
        
        assert_eq!(client.base_url, "http://169.254.169.254");
        assert_eq!(client.token_ttl, 30);
        assert_eq!(client.api_version, "2020-01-01");
    }
}


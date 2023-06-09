use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client;
use ec2_imds::{IMDSClient, IMDSCommand, IPVersion};
use ec2_instance_metadata
fn rotate_ec2_ip<'a>() {
    let client = IMDSClient::new(IPVersion::Ipv4, None, None);
    let a = client.send_command(IMDSCommand::PublicIpv4);
    let instance_metadata = client.get().expect("Couldn't get the instance metadata.");
}

#[tokio::main]
async fn main() -> Result<(), Box<std::error::Error>> {

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // client.describe_addresses().allocation_ids(input);

    client.allocate_address();



    Ok(())
}

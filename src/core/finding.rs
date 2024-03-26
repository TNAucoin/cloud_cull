const EC2_VOLUME_FINDING_ID: &str = "AWS::EC2::Volume";
const LOG_GROUP_FINDING_ID: &str = "AWS::Logs::LogGroup";
const EC2_ELASTIC_IP_FINDING_ID: &str = "AWS::EC2::EIP";

const EBS_VOLUME_RESOURCE_TYPE: &str = "EBS::Volume";
const LOG_GROUP_RESOURCE_TYPE: &str = "Logs::LogGroup";
const EC2_ELASTIC_IP_RESOURCE_TYPE: &str = "EC2::EIP";

#[derive(Debug, Clone)]
pub struct Finding {
    pub id: String,
    pub resource_arn: String,
    pub resource_type: String,
}

pub enum FindingId {
    EbsVolume,
    LogGroup,
    ElasticIp,
}

impl Finding {
    pub fn new(finding_id: FindingId, resource_arn: String) -> Self {
        let (id, resource_type) = match finding_id {
            FindingId::EbsVolume => (EC2_VOLUME_FINDING_ID, EBS_VOLUME_RESOURCE_TYPE),
            FindingId::LogGroup => (LOG_GROUP_FINDING_ID, LOG_GROUP_RESOURCE_TYPE),
            FindingId::ElasticIp => (EC2_ELASTIC_IP_FINDING_ID, EC2_ELASTIC_IP_RESOURCE_TYPE),
        };
        Self {
            id: id.to_string(),
            resource_arn,
            resource_type: resource_type.to_string(),
        }
    }
}

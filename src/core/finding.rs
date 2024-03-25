#[derive(Debug, Clone)]
pub struct Finding {
    pub finding_id: String,
    pub resource_arn: String,
    pub resource_type: String,
}

pub enum FindingId {
    EbsVolume,
}

impl Finding {
    pub fn new(finding_id: FindingId, resource_arn: String) -> Self {
        let id = match finding_id {
            FindingId::EbsVolume => "AWS::EC2::Volume".to_string(),
        };
        let resource_type = match finding_id {
            FindingId::EbsVolume => "EBS::Volume".to_string(),
        };
        Self {
            finding_id: id,
            resource_arn,
            resource_type,
        }
    }
}

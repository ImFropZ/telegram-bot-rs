pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

pub struct EncryptedPassportElement {
    pub r#type: String,
    pub data: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub files: Option<Vec<PassportFile>>,
    pub front_side: Option<PassportFile>,
    pub reverse_side: Option<PassportFile>,
    pub selfie: Option<PassportFile>,
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i32,
    pub file_date: i32,
}


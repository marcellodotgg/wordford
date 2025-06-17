use crate::orgs::org_repository::OrgRepository;

pub struct OrgService {
    org_repository: OrgRepository,
}

impl OrgService {
    pub fn new(org_repository: OrgRepository) -> Self {
        OrgService { org_repository }
    }

    pub async fn find_by_id(&self, org_id: &str) -> Result<Option<String>, sqlx::Error> {
        self.org_repository.find_by_id(org_id).await
    }

    pub async fn create_org(&self, name: &str) -> Result<String, sqlx::Error> {
        self.org_repository.create_org(name).await
    }

    pub async fn delete_org(&self, org_id: &str) -> Result<(), sqlx::Error> {
        self.org_repository.delete_org(org_id).await
    }
}

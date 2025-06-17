use crate::orgs::org_repository::OrgRepository;

pub struct OrgService {
    org_repository: OrgRepository,
}

impl OrgService {
    pub fn new(org_repository: OrgRepository) -> Self {
        OrgService { org_repository }
    }
}

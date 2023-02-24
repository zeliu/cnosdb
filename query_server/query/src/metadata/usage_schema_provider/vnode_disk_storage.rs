use std::sync::Arc;

use coordinator::service::CoordinatorRef;
use datafusion::datasource::TableProvider;
use meta::MetaClientRef;
use models::auth::user::User;
use spi::Result;

use crate::metadata::usage_schema_provider::{
    create_usage_schema_view_table, UsageSchemaTableFactory,
};

pub const USAGE_SCHEMA_VNODE_DISK_STORAGE: &str = "vnode_disk_storage";
pub struct VnodeDiskStorage {}

impl UsageSchemaTableFactory for VnodeDiskStorage {
    fn table_name(&self) -> &'static str {
        USAGE_SCHEMA_VNODE_DISK_STORAGE
    }

    fn create(
        &self,
        user: &User,
        coord: CoordinatorRef,
        meta: MetaClientRef,
        default_catalog: MetaClientRef,
    ) -> Result<Arc<dyn TableProvider>> {
        create_usage_schema_view_table(
            user,
            coord,
            meta,
            USAGE_SCHEMA_VNODE_DISK_STORAGE,
            default_catalog,
        )
    }
}
use dirtybase_orm::base::manager::Manager;

// The table that will hold migration information
async fn setup_migration_table(manager: &Manager) {
    let name = "_core_migration";
    if !manager.has_table(name).await {
        manager
            .table(name, |table| {
                // id
                table.id(None);
                // migration name
                table.string("name");
                // created at
                table.created_at();
                // deleted at
                table.updated_at();
            })
            .await;
    }
}

// The table that will hold file metadata
async fn setup_file_metadata_table(manager: &Manager) {
    let name = "_core_file_meta";
    if !manager.has_table(name).await {
        manager
            .table(name, |table| {
                // internal_id
                // id
                table.id_set();
                // external_id
                table.ulid("external_id").set_is_nullable(false);
                // meta
                table.json("meta");
                // timestamp
                table.timestamps();
            })
            .await;
    }
}

// The table that will hold company's tenets
async fn setup_company_table(manager: &Manager) {
    let name = "_core_company";
    if !manager.has_table(name).await {
        manager
            .table(name, |table| {
                // internal_id
                // id
                table.id_set();
                // name
                table.string("name");
                // description
                table.sized_string("description", 512);
                // timestamp
                table.timestamps();
            })
            .await;
    }
}

// The global user table
async fn setup_users_table(manager: &Manager) {
    let name = "_core_users";
    if !manager.has_table(name).await {
        manager
            .table(name, |table| {
                table.id_set();
                table.string("email");
                table.string("username");
                // authentication method !?!?!
                table.timestamps();
                table.soft_deletable();
            })
            .await;
    }
}

// The global roles table
async fn setup_roles_table(manager: &Manager) {
    let name = "_app_core_role";
    if !manager.has_table(name).await {
        manager
            .table(name, |table| {
                // internal_id
                // id
                table.id_set();
                // company_id
                table
                    .ulid("company_id")
                    .set_is_nullable(false)
                    .references_without_cascade_delete("_core_company", "id");
                // name
                table.string("name");
                // blame
                table.blame(None);
                // timestamps
                table.timestamps();
            })
            .await;
    }
}

// A user role
async fn setup_role_users_table(_manager: &Manager) {
    let name = "_core_role_user";
    // if manager.has_table(name).await {
    //     manager.table(name, |table|{

    //     });
    // }
}

async fn setup_applications_table(_manager: &Manager) {}

// The table that will contain the "collections" definitions
async fn setup_schema_table(_manager: &Manager) {}

pub(crate) async fn create_data_tables(manager: &Manager) {
    setup_migration_table(manager).await;
    setup_file_metadata_table(manager).await;
    setup_company_table(manager).await;
    setup_users_table(manager).await;
    setup_roles_table(manager).await;
    setup_role_users_table(manager).await;
    setup_applications_table(manager).await;
    setup_schema_table(manager).await;
}

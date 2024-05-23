## Database Migrations

Newer database changes introduced during development are applied as follows:

1. Create the change using `./db_add_change.sh {change-name}`.<br/>
   Ex: `./db_add_change.sh create_table_users_credentials`

2. Populate the generated file.<br/>

3. Apply the change using `./db_apply_changes.sh`.

/*
	File with the postgres queries
*/

pub const GET_TABLES: &str = "
			SELECT table_catalog as db_name, table_schema,table_name
            FROM information_schema.tables
            WHERE table_schema = any($1);";

pub const GET_ATTRIBUTES: &str = "
			SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_schema = $1 AND table_name = $2;";

pub const GET_PRIMARY_KEYS: &str = "
			SELECT tc.table_schema,tc.table_name,c.column_name
            FROM information_schema.table_constraints tc 
            JOIN information_schema.constraint_column_usage AS ccu USING (constraint_schema, constraint_name) 
            JOIN information_schema.columns AS c ON c.table_schema = tc.constraint_schema
              AND tc.table_name = c.table_name AND ccu.column_name = c.column_name
            WHERE constraint_type = 'PRIMARY KEY' AND tc.table_schema = $1
            AND tc.table_name = $2;
";

pub const GET_FOREIGN_KEYS: &str = "
 			SELECT
                tc.table_schema, 
                tc.table_name, 
                kcu.column_name, 
                ccu.table_schema AS foreign_table_schema,
                ccu.table_name AS foreign_table_name,
                ccu.column_name AS foreign_column_name 
            FROM information_schema.table_constraints AS tc 
            JOIN information_schema.key_column_usage AS kcu
                ON tc.constraint_name = kcu.constraint_name
                AND tc.table_schema = kcu.table_schema
            JOIN information_schema.constraint_column_usage AS ccu
                ON ccu.constraint_name = tc.constraint_name
            WHERE tc.constraint_type = 'FOREIGN KEY' AND
            tc.table_schema = any($1) AND ccu.table_schema = any($1);";
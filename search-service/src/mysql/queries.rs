/*
	File with the mysql queries
*/

pub const GET_TABLES: &str = "
			SELECT table_schema, table_name
			FROM information_schema.tables
			WHERE table_schema IN ( :allowed_schemas )";

pub const GET_ATTRIBUTES: &str = "
			SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_schema = :table_schema AND table_name = :table_name ;";

pub const GET_PRIMARY_KEYS: &str = "
			SELECT k.column_name
			FROM information_schema.table_constraints t
			JOIN information_schema.key_column_usage k
			USING(constraint_name,table_schema,table_name)
			WHERE t.constraint_type='PRIMARY KEY'
			  AND t.table_schema=:table_schema
			  AND t.table_name=:table_name;
";

pub const GET_FOREIGN_KEYS: &str = "
			SELECT tc.table_schema,tc.table_name,kcu.column_name,
			kcu.referenced_table_schema,kcu.referenced_table_name,
			kcu.referenced_column_name
			FROM information_schema.table_constraints AS tc
			JOIN information_schema.key_column_usage AS kcu
				ON tc.constraint_name = kcu.constraint_name
				AND tc.table_schema = kcu.table_schema
			WHERE tc.constraint_type= 'FOREIGN KEY'
			AND tc.table_schema IN ( :allowed_schemas )
			AND kcu.referenced_table_schema IN ( :allowed_schemas );
";
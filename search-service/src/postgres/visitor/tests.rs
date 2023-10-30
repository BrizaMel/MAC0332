#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use crate::query_representation::intermediary::Command;

    use crate::query_representation::intermediary::tests::tests::{
        create_composite_command, create_simple_command,
    };

    use crate::postgres::visitor::PostgresVisitor;

    use crate::traits::Component;

    use anyhow::Error;

    #[test]
    fn test_visitor_architecture() -> Result<(), Error> {
        let simple_command = create_simple_command()?;
        let composite_command = create_composite_command()?;

        let sc_return = Command::SingleCommand(simple_command)
            .accept(vec!["projection".to_string()], Arc::new(PostgresVisitor))?;
        let cc_return = Command::CompositeCommand(composite_command)
            .accept(vec!["projection".to_string()], Arc::new(PostgresVisitor))?;

        assert_eq!(
            sc_return,
            "Command to query not implemented yet".to_string()
        );
        assert_eq!(
            cc_return,
            "Command to query not implemented yet".to_string()
        );

        Ok(())
    }
}

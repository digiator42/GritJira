use gritshield::{
    GritComponent,
    database::repository::{CustomQuerySpec, JqlCompiler},
    security::xss::Sanitizer,
};
use sea_orm::{DbBackend, Statement};

#[derive(Clone, GritComponent)]
pub struct JqlParser {}

impl JqlParser {

    /// Compiles a JQL/SQL string into a SeaORM Statement
    pub fn compile_jql(
        &self,
        raw_jql: &str,
        base_table: &str,
        backend: DbBackend,
    ) -> Result<Statement, String> {
        // If raw query lacks SELECT/FROM, construct a default SELECT for issues
        let query_str = if !raw_jql.to_lowercase().contains("select") {
            format!("SELECT * FROM {} WHERE {}", base_table, raw_jql)
        } else {
            raw_jql.to_string()
        };

        let decoded_query = Sanitizer::url_decode(&query_str);
        println!("==> {decoded_query}");

        // Parse query specification
        let spec = CustomQuerySpec::parse_from_str(&decoded_query)?;

        // Compile specification into SeaORM Statement
        Ok(JqlCompiler::compile(&spec, backend))
    }
}

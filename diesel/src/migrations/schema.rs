table! {
    __diesel_schema_migrations (version) {
        version -> BigInt,
        run_on -> Timestamp,
    }
}

pub struct NewMigration(pub i64);

use expression::AsExpression;
use expression::grouped::Grouped;
use expression::helper_types::AsExpr;
use {Insertable, types};

impl<'a> Insertable<__diesel_schema_migrations::table> for &'a NewMigration {
    type Columns = __diesel_schema_migrations::version;
    type Values = Grouped<AsExpr<i64, Self::Columns>>;

    fn columns() -> Self::Columns {
        __diesel_schema_migrations::version
    }

    fn values(self) -> Self::Values {
        Grouped(AsExpression::<types::BigInt>::as_expression(self.0))
    }
}

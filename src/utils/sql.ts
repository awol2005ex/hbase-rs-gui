const get_spark_tv_sql_impl = async (tablename: string, columns: string[]) => {
  let columns_str = "";
  if (columns.length > 0) {
    columns_str =
      "," +
      columns
        .map((c) => {
          return `${c.split(":")[1]} ${c} String`;
        })
        .join(",");
  }
  let sql = `CREATE OR REPLACE TEMPORARY VIEW spark_tv_${(
    tablename as string
  ).replace(":", "_")}
            USING org.apache.hadoop.hbase.spark
  OPTIONS(
    "hbase.table"="${tablename as string}",
    "hbase.columns.mapping"="rowKey String :key${columns_str}",
    "hbase.spark.use.hbasecontext"="false"
  )
    `;
  return sql;
};

const create_flink_table_sql_impl = async (
  tablename: string,
  columns: string[],
  columnFamilies: string[]
) => {
  let cf_str = "";
  if (columnFamilies.length > 0) {
    cf_str = columnFamilies
      .map((cf) => {
        let columns_str = columns
          .filter((c) => {
            return c.startsWith(cf + ":");
          })
          .map((c) => {
            return `${c.split(":")[1]} ${c} String`;
          })
          .join(",");
        return `${cf} ROW<${columns_str}>,`;
      })
      .join(",\n");
  }

  let sql = `
    CREATE TABLE if not exists flink_table_${(tablename as string).replace(
      ":",
      "_"
    )} (
   rowkey  STRING,
  ${cf_str}
   PRIMARY KEY (rowkey ) NOT ENFORCED
  ) WITH (
   'connector' = 'hbase-2.2',
   'table-name' = '${tablename as string}',
   'zookeeper.quorum' = 'xxx:2181',
   'hadoop.security.authentication'='kerberos',
   'hbase.security.authentication'='kerberos',
   'hbase.master.kerberos.principal'='hive/_HOST@XXX.COM',
   'hbase.regionserver.kerberos.principal'='hive/_HOST@XXX.COM'
  )
    `;
  return sql;
};

const create_hive_sql_impl = async (tablename: string, columns: string[]) => {
  let columns_str2 = "";
  if (columns.length > 0) {
    columns_str2 = "," + columns.join(",");
  }

  let columns_str1 = "";
  if (columns.length > 0) {
    columns_str1 = columns
      .map((c) => {
        return `,${c.split(":")[1]} string`;
      })
      .join("\n");
  }

  let sql = `CREATE EXTERNAL TABLE  ${(tablename as string).replace(":", ".")} (
    rowkey string COMMENT 'rowkey'
  ${columns_str1}
    )
  ROW FORMAT SERDE
    'org.apache.hadoop.hive.hbase.HBaseSerDe'
  STORED BY
    'org.apache.hadoop.hive.hbase.HBaseStorageHandler'
  WITH SERDEPROPERTIES (
    'hbase.columns.mapping'=':key${columns_str2}')
  TBLPROPERTIES (
    'TRANSLATED_TO_EXTERNAL'='TRUE',
    'bucketing_version'='2',
    'external.table.purge'='FALSE',
    'hbase.table.name'='${tablename as string}')
     `;

  return sql;
};

export {
  get_spark_tv_sql_impl,
  create_flink_table_sql_impl,
  create_hive_sql_impl,
};

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
  columnFamilies: string[],
  hbaseConfig :Map<string,string>,
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

  let prop_str="";
  if(hbaseConfig.has("hbase.zookeeper.quorum") &&  hbaseConfig.has("hbase.zookeeper.property.clientPort")){

    const zk=hbaseConfig.get("hbase.zookeeper.quorum")?.split(",").map(q=>q+":"+hbaseConfig.get("hbase.zookeeper.property.clientPort")).join(",");
      prop_str=prop_str+ `\n,'zookeeper.quorum' = '${zk}'`

      
  }
  if((hbaseConfig.get("hadoop.security.authentication")||"")=="kerberos") {
    prop_str=prop_str+ `\n,'hadoop.security.authentication' = 'kerberos'`
    prop_str=prop_str+ `\n,'hbase.security.authentication' = 'kerberos'`
    prop_str=prop_str+ `\n,'hbase.master.kerberos.principal' = '${ hbaseConfig.get("hadoop.security.kerberos.principal")?.replace("@","/_HOST@") }'`
    prop_str=prop_str+ `\n,'hbase.regionserver.kerberos.principal' = '${ hbaseConfig.get("hadoop.security.kerberos.principal")?.replace("@","/_HOST@") }'`
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
   'table-name' = '${tablename as string}'
   ${prop_str}
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

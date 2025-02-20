
<h1 align="center">
  HBase GUI
  <br>
</h1>

<h4 align="center">A HBase File GUI Software.</h4>


<p align="center">
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#download">Download</a> •
  <a href="#credits">Credits</a> •
  <a href="#related">Related</a> •
  <a href="#license">License</a>
</p>

![screenshot](doc\screenshot.png)

## Key Features

* Cross platform
  - Windows, macOS and Linux ready.

## How To Use

To clone and run this application, you'll need [Git](https://git-scm.com) and [Node.js](https://nodejs.org/en/download/) (which comes with [npm](http://npmjs.com)) installed on your computer. From your command line:



```bash
# git bash
# Clone this repository
$ git clone https://github.com/awol2005ex/hbase-rs-gui.git

# Go into the repository
$ cd hbase-rs-gui

# Install dependencies
$ npm install

# Run the app
$ npm run tauri build

# close the repository for java operation

$ cd ..

$ git clone https://github.com/awol2005ex/hbase-oper.git

$ cd hbase-oper

$ ./gradlew shadowJar

$ cp build/libs/hbase-oper-all.jar ../hbase-rs-gui/src-tauri/target/release/

$ zip hbase-rs-gui.zip build/libs/hbase-oper-all.jar ../hbase-rs-gui/src-tauri/target/release/hbase-rs-gui.exe
```



Configure example :

config:
```json
{
	"hbase.rootdir": "hdfs://nameservice1/hbase",
	"hbase.zookeeper.quorum": "nn3,nn2,nn1",
	"hbase.zookeeper.property.clientPort": "2181",
	"hbase.master.kerberos.principal": "hbase/_HOST@XXX.COM",
	"hadoop.security.authentication": "kerberos",
	"hadoop.security.authorization": "true",
	"hbase.security.authentication": "kerberos",
	"hbase.defaults.for.version.skip": "true",
	"hbase.regionserver.kerberos.principal": "hbase/_HOST@XXX.COM",
	"hbase.thrift.kerberos.principal": "hbase/_HOST@XXX.COM",
	"hbase.thrift.ssl.enabled": "false",
	"hbase.rpc.protection": "authentication",
	"hadoop.security.kerberos.keytab": "/tmp/hive@XXX.COM.keytab",
	"hadoop.security.kerberos.principal": "hive@XXX.COM"
}
```

env:
```json
{
   "java.security.krb5.conf":"D:/MIT/krb5.ini",
   "javax.security.auth.useSubjectCredsOnly":"false"
}
```

## Download

You can [download](https://github.com/awol2005ex/hbase-rs-gui/releases/tag/v0.1.0) the latest installable version of hbase-rs-gui for Windows  (JDK 17 or higher on PATH ENV Variable)




## License

Apache License 2.0

---
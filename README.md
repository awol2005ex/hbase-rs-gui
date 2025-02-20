
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


## Download

You can [download](https://github.com/awol2005ex/hbase-rs-gui/releases/tag/v0.1.0) the latest installable version of hbase-rs-gui for Windows 




## License

Apache License 2.0

---
const { Binary } = require("binary-install");
const os = require("os");

function getPlatform() {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") return "win64";
  if (type === "Linux" && arch === "x64") return "linux";
  if (type === "Darwin" && arch === "x64") return "mac";

  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getBinary() {
  const platform = getPlatform();
  const version = require("../package.json").version;
  const url = `https://github.com/josephsawaya/aws-serverless-toolkit-cli/releases/download/v${version}/ast-cli-${platform}.tar.gz`;
  const name = "ast-cli";
  return new Binary(url, { name });
}

module.exports = getBinary;

import fs from "fs";
const packageFileContent = fs.readFileSync("js/pkgs/package.json", "utf-8");
const packageJSON = JSON.parse(packageFileContent);
packageJSON.type = "module";
packageJSON.main = packageJSON.module;
fs.writeFileSync("js/pkgs/package.json", JSON.stringify(packageJSON, null, 2), "utf-8");

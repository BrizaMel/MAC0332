export function download(dictstring: string, filename: string) {
  const dict = [dictstring];
  var blobFile = new Blob(dict, {
    type: "application/json;charset=utf-8",
  });

  var url = window.URL || window.webkitURL;
  const link = url.createObjectURL(blobFile);
  var file = document.createElement("a");
  file.download = `${filename}.json`;
  file.href = link;
  document.body.appendChild(file);
  file.click();
  document.body.removeChild(file);
}

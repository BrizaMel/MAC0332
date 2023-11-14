export function download(dictstring: string, filename: string) {
  const arrdictstring = [dictstring];
  var blobFile = new Blob(arrdictstring, {
    type: "application/json;charset=utf-8",
  });

  var url = window.URL || window.webkitURL;
  const link = url.createObjectURL(blobFile);
  var a = document.createElement("a");
  a.download = `${filename}.json`;
  a.href = link;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
}

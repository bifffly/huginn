import Net from 'net';
import URL from 'url';

function getUrlData(url: string) {
  if (!url.includes('://')) {
    url = `odin://${url}`;
  }
  let parsed = URL.parse(url);
  return {
    hostname: parsed.hostname ?? '',
    path: parsed.path
  };
}

export function preflight(url: string) {
  let parsed = getUrlData(url);
  return new Promise((resolve, reject) => {
    let client = new Net.Socket();
    client.connect(1866, parsed.hostname, () => {
      client.write(`odin\tprefligh\t\t${parsed.path}`);
    });
    client.on('data', (res) => {
      resolve(res.toString());
    });
  });
}

export function pull(url: string) {
  let parsed = getUrlData(url);
  return new Promise((resolve, reject) => {
    let client = new Net.Socket();
    client.connect(1866, parsed.hostname, () => {
      client.write(`odin\tpull\t${parsed.path}`);
    });
    client.on('data', (res) => {
      resolve(res.toString());
    });
  });
}

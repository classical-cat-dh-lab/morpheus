import {runEngine, identity} from '../engine/runtime.mjs';

self.onmessage = async ({data: request}) => {
  try {
    self.postMessage({type: 'progress', id: request.id, message: 'Loading morphology data…'});
    const [wasmBinary, data] = await Promise.all(['morpheus.wasm', 'morpheus.data'].map(async name => {
      const response = await fetch(new URL('../engine/' + name, import.meta.url), {signal: AbortSignal.timeout(30000)});
      if (!response.ok) throw new Error(`Could not load ${name} (${response.status}).`);
      const bytes = await response.arrayBuffer();
      const digest = [...new Uint8Array(await crypto.subtle.digest('SHA-256', bytes))].map(x => x.toString(16).padStart(2, '0')).join('');
      if (bytes.byteLength !== identity.assets[name].bytes || digest !== identity.assets[name].sha256) throw new Error(`The saved ${name} is incomplete or damaged. Save the offline copy again.`);
      return bytes;
    }));
    self.postMessage({type: 'progress', id: request.id, message: 'Analyzing…'});
    const result = await runEngine(request, {wasmBinary, data});
    self.postMessage({type: 'result', id: request.id, result});
  } catch (error) {
    self.postMessage({type: 'error', id: request.id, message: String(error.message ?? error)});
  }
};

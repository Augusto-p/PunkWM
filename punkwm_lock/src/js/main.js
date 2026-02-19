async function setWallpaer(bg) {
    const contents = await window.__TAURI_PLUGIN_FS__.readFile(bg);
    const blob = new Blob([contents], { type: 'image/jpeg' });
    const blobUrl = URL.createObjectURL(blob);
    document.body.style.backgroundImage = `url('${blobUrl}')`;
}

// setWallpaer("https://preview.redd.it/forest-3840-x-2160-ai-v0-ie48435jkjfg1.jpeg?auto=webp&s=a26d2f188d943020d368ad826c3c2995b0562774")

emit_system_start();
window.focus()
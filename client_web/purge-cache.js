(async () => {
  const root = await navigator.storage.getDirectory();

  for await (const [name, _] of root.entries()) {
    await root.removeEntry(name, { recursive: true });
  }
})();

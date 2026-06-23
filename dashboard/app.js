const file = document.getElementById("file");

function dump(obj) {
  return JSON.stringify(obj, null, 2);
}

file.addEventListener("change", async () => {
  const f = file.files[0];
  if (!f) return;

  const text = await f.text();
  const report = JSON.parse(text);

  const summary = report.summary || {};
  document.getElementById("score").textContent = summary.score ?? "--";
  document.getElementById("summary").textContent = dump(summary);
  document.getElementById("findings").textContent = dump((report.findings || []).slice(0, 25));
  document.getElementById("artifacts").textContent = dump((report.artifacts || []).slice(0, 25));
  document.getElementById("timeline").textContent = dump((report.timeline || []).slice(0, 25));
});

const views = [...document.querySelectorAll(".view")];
const navButtons = [...document.querySelectorAll(".nav")];
const log = document.getElementById("log");

const trimByProfile = {
  standard: ["Stop OneDrive background sync", "Stop SysMain prefetch service"],
  aggressive: ["Stop OneDrive background sync", "Stop SysMain prefetch service", "Suspend desktop shell during workload"],
  recovery: ["Restart SysMain service", "Restore desktop shell"]
};

function showView(id) {
  views.forEach((view) => view.classList.toggle("active", view.id === id));
  navButtons.forEach((button) => button.classList.toggle("active", button.dataset.view === id));
}

function appendLog(message) {
  log.textContent += `\n${message}`;
  log.scrollTop = log.scrollHeight;
}

function renderPlan() {
  const profile = document.getElementById("profile").value;
  const ram = Number(document.getElementById("ram").value || 0);
  const vram = Number(document.getElementById("vram").value || 0);
  const vcpu = Number(document.getElementById("vcpu").value || 0);
  const availableRam = 7.4;
  const localVram = 2;
  const localCpu = 12;

  document.getElementById("ramMetric").textContent = `${availableRam.toFixed(1)} GB`;
  document.getElementById("cpuMetric").textContent = `${localCpu}`;

  const ramDeficit = Math.max(0, ram - availableRam);
  const vramDeficit = Math.max(0, vram - localVram);
  const cpuDeficit = Math.max(0, vcpu - localCpu);
  const cloudNeed = ramDeficit + vramDeficit + cpuDeficit > 0 ? "Required" : "Low";
  document.getElementById("cloudMetric").textContent = cloudNeed;

  const actions = document.getElementById("trimActions");
  actions.innerHTML = "";
  trimByProfile[profile].forEach((action) => {
    const item = document.createElement("li");
    item.textContent = action;
    actions.appendChild(item);
  });

  const bridgePlan = document.getElementById("bridgePlan");
  bridgePlan.innerHTML = "";
  [
    ["RAM Burst", `${ramDeficit.toFixed(1)} GB`],
    ["vRAM Burst", `${vramDeficit.toFixed(1)} GB`],
    ["vCPU Burst", `${cpuDeficit}`],
    ["Provider", document.getElementById("provider").value],
    ["Storage", "20 GB"],
    ["Auth", "API key pending"]
  ].forEach(([label, value]) => {
    const cell = document.createElement("div");
    cell.innerHTML = `<span class="label">${label}</span><br><strong>${value}</strong>`;
    bridgePlan.appendChild(cell);
  });

  appendLog(`[plan] profile=${profile} ram=${ram} vram=${vram} vcpu=${vcpu} cloud=${cloudNeed}`);
}

navButtons.forEach((button) => {
  button.addEventListener("click", () => showView(button.dataset.view));
});

document.getElementById("plan").addEventListener("click", renderPlan);
renderPlan();

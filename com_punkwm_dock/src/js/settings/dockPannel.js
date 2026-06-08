const DockWidth = document.getElementById("DockWidth")
const PannelWidth = document.getElementById("PannelWidth")

function DockWidthSet(mode) {
  switch (mode) {
    case "++":
      DockWidth.value++;
      break;
  case "--":
      DockWidth.value--;
      break;
  default:
      break;
  }
  DockWidthInput()
  
}

function DockWidthInput() {
  DockWidth.value = DockWidth.value.replace("-","")
  if (parseInt(DockWidth.value) < parseInt(DockWidth.min)) {
    DockWidth.value = DockWidth.min;
  }
  if (DockWidth.value == "") {
    DockWidth.value = DockWidth.min;
  }
  setDockWidth(DockWidth.value)


}

function setDockWidth(value, save = true) {
  Storage.setItem("Dock Width", value)  
  if (save) {DockPannel.SaveWidthDock(value);}

}

function getDockWidth() {
  return Storage.getItem("Dock Width") ?? 1  
}



function PannelWidthSet(mode) {
  switch (mode) {
    case "++":
      PannelWidth.value++;
      break;
  case "--":
      PannelWidth.value--;
      break;
  default:
      break;
  }
  PannelWidthInput()
  
}

function PannelWidthInput() {
  PannelWidth.value = parseInt(PannelWidth.value.replace("-",""))
  if (parseInt(PannelWidth.value) < parseInt(PannelWidth.min)) {
    PannelWidth.value = PannelWidth.min;
  }
  if (PannelWidth.value == "") {
    PannelWidth.value = PannelWidth.min;
  }
  setPannelWidth(PannelWidth.value)
}

function setPannelWidth(value, save = true) {
  Storage.setItem("Pannel Width", value);
  if (save) {DockPannel.SaveWidthPannel(value);}
}

function getPannelWidth() {
  return Storage.getItem("Pannel Width") ?? 1  
}

DockWidth.value = getDockWidth()
PannelWidth.value = getPannelWidth()


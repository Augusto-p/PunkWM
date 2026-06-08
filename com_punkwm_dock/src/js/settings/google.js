const Credentials = document.getElementById("CredentialsJSON");
const CredentialsInput = document.getElementById("CredentialsInput");
const GoogleMaxEvents = document.getElementById("GoogleMaxEvents");
let Scopes = []

async function getScopesDir() {
  let response = await fetch(`assets/settings/JSONs/Scopes.json`, { method: "GET", });
  let data = await response.json();
  Scopes = data.map(v => ({ original: v, lower: v.toLowerCase() }));
}

getScopesDir();


const GoogleScopesAutocomplete = document.getElementById("GoogleScopesAutocomplete");
const GoogleScopes = document.getElementById("GoogleScopes")
function syntaxHighlight(json) {
  json = JSON.parse(`"${json}"`);
  json = JSON.stringify(json, null, 2);

  // 🔹 escapar HTML primero
  json = json
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");

  // 🔹 1. colorear tipos (SIN tocar llaves)
  json = json.replace(
    /("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|\b\d+(\.\d+)?\b)/g,
    function (match) {
      let cls = "number";
      if (/^"/.test(match)) {
        cls = /:$/.test(match) ? "key" : "string";
      } else if (/true|false/.test(match)) {
        cls = "boolean";
      } else if (/null/.test(match)) {
        cls = "null";
      }
      return `<span class="${cls}">${match}</span>`;
    }
  );

  // 🔹 2. ahora recorrer y colorear llaves SIN romper spans
  let depth = -1;
  let result = "";
  let inTag = false;

  for (let i = 0; i < json.length; i++) {
    let char = json[i];

    // detectar si estamos dentro de un tag HTML
    if (char === "<") inTag = true;
    if (char === ">") {
      inTag = false;
      result += char;
      continue;
    }

    if (inTag) {
      result += char;
      continue;
    }

    // 👉 solo tocar texto real, no HTML
    if (char === "{" || char === "[" || char === "(") {
      depth++;
      result += `<span class="brace depth-${depth % 3}">${char}</span>`;
    }
    else if (char === "}" || char === "]" || char === ")") {
      result += `<span class="brace depth-${depth % 3}">${char}</span>`;
      depth--;
    }
    else {
      result += char;
    }
  }

  return result;
}


function LoadCredentials(Credentialstext) {
  Credentials.innerHTML = syntaxHighlight(Credentialstext);
  Google.SaveCredentials(Credentialstext)
}

CredentialsInput.addEventListener("change", function (event) {
  const file = event.target.files[0];

  if (!file) return;

  const reader = new FileReader();

  reader.onload = function (e) {
    try {
      LoadCredentials(JSON.parse(e.target.result))
    } catch (error) {
      console.error("Error al parsear JSON:", error);
    }
  };

  reader.readAsText(file);
});

function getGoogleMaxEvents() {
  return Storage.getItem("Google-Max-Events") ?? 1
}

function setGoogleMaxEvents(value,) {
  Storage.setItem("Google-Max-Events", value);
  Google.SaveMaxEventsView(value);
}

function GoogleMaxEventsSet(mode) {
  switch (mode) {
    case "++":
      GoogleMaxEvents.value++;
      break;
    case "--":
      GoogleMaxEvents.value--;
      break;
    default:

      console.log(GoogleMaxEvents.value);

      break;
  }
  GoogleMaxEventsInput()

}

function GoogleMaxEventsInput() {
  GoogleMaxEvents.value = GoogleMaxEvents.value.replace("-", "")
  if (parseInt(GoogleMaxEvents.value) > parseInt(GoogleMaxEvents.max)) {
    GoogleMaxEvents.value = GoogleMaxEvents.max;
  }
  if (parseInt(GoogleMaxEvents.value) < parseInt(GoogleMaxEvents.min)) {
    GoogleMaxEvents.value = GoogleMaxEvents.min;
  }
  setGoogleMaxEvents(GoogleMaxEvents.value)
}


let debounceTimer;

function debounce(callback, delay) {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(callback, delay);
}

function GoogleScopesInputInput(elem) {
  const query = elem.value.toLowerCase();
  debounce(() => {
    const matches = Scopes
      .filter(item => item.lower.includes(query))
      .slice(0, 12)
      .map(item => item.original);

    GoogleScopesAutoCoplete(matches);
  }, 100);
}

function GoogleScopesAutoCoplete(terms) {
  GoogleScopesAutocomplete.innerHTML = "";
  terms.forEach(scope => {
    GoogleScopesAutocomplete.innerHTML += `<span onclick="GoogleScopeAdd('${scope}')">${scope}</span>`;
  });

}

function GoogleScopeAdd(scope) {
  if (scope == "") {
    scope = GoogleScopesInput.value;
    GoogleScopesInput.innerHTML = "";
    document.body.focus()
  }
  GoogleScopesInput.value = "";

  let scopeLocal = JSON.parse(Storage.getItem("Google-Scopes")) ?? [];
  scopeLocal.push(scope);
  Storage.setItem("Google-Scopes", JSON.stringify(scopeLocal));
  Google.addScope(scope);
  viewGoogleScopes()
}

function GoogleScopeDel(scope) {
  let scopeLocal = JSON.parse(Storage.getItem("Google-Scopes")) ?? [];
  scopeLocal.pop(scope);
  Storage.setItem("Google-Scopes", JSON.stringify(scopeLocal));
  Google.popScope(scope);
  viewGoogleScopes()
}

function viewGoogleScopes() {
  let scopeLocal = JSON.parse(Storage.getItem("Google-Scopes")) ?? [];
  GoogleScopes.innerHTML = ""
  scopeLocal.forEach(scope => {
    GoogleScopes.innerHTML += `<div class="item">
                    <span>${scope}</span>
                    <button onclick="GoogleScopeDel('${scope}')"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 -960 960 960" >
                            <path
                                d="M280-120q-33 0-56.5-23.5T200-200v-520h-40v-80h200v-40h240v40h200v80h-40v520q0 33-23.5 56.5T680-120H280Zm400-600H280v520h400v-520ZM360-280h80v-360h-80v360Zm160 0h80v-360h-80v360ZM280-720v520-520Z" />
                        </svg></button>
                </div>`

  });

}



GoogleMaxEvents.value = getGoogleMaxEvents();
viewGoogleScopes();
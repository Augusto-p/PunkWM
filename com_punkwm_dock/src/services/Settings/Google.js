class Google extends Sender {
    static async SaveMaxEventsView(value){super.Emit("Google", "Save Max Events View", {"Max":value});}
    static async SaveCredentials(value){super.Emit("Google", "Save Credentials", {"Credentials":value});}
    static async addScope(value){super.Emit("Google", "Add Scope", {"Scope":value});}
    static async popScope(value){super.Emit("Google", "Pop Scope", {"Scope":value});}
    static Load(Events, Scopes, Credentials){
        Storage.setItem("Google-Max-Events", Events);
        GoogleMaxEvents.value = getGoogleMaxEvents();
        Storage.setItem("Google-Scopes", JSON.stringify(Scopes));
        viewGoogleScopes();
        LoadCredentials(Credentials);
    }
    
}
class Language extends Sender {
    static async Save(id){super.Emit("Language", "Save", {"Id": id});}
    static Load(lang){
        setLang(lang, false); 
        LoadLagUI();
    }
    
}
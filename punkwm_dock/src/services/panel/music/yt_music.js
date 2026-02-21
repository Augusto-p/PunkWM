function Load_Visitor_Data(code, roll=null) {
    window.sessionStorage.setItem("YT:VisitorData", code)
    if (roll) {
        window.sessionStorage.setItem("YT:RolloutToken", roll)
    }
}
function getVisitorData() {
    return window.sessionStorage.getItem("YT:VisitorData")
    
}
function getRolloutToken() {
    return window.sessionStorage.getItem("YT:RolloutToken")
    
}
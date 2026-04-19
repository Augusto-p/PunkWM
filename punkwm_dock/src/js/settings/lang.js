async function getLangDir(lang) {
    let response = await fetch(`langs/${lang}.json`, { method: "GET",});
    let data = await response.json();
    return data
}


async function LoadLagUI() {
    Lang = await getLangDir(getLang());

    let elements = document.querySelectorAll("[data-Lang]")
    elements.forEach(ele => {
        let tag = ele.dataset.lang
        let tagSplit = tag.split(".")
        let Word = "";
        for (let index = 0; index < tagSplit.length; index++) {
            if (index == 0) {
                Word = Lang[tagSplit[0]]
                continue
            }
            Word = Word[tagSplit[index]];
        }
        if (ele.tagName == "INPUT" && ele.type== "text") {
            ele.placeholder = ele.placeholder.replace(tagSplit.reverse()[0], Word);        
            
        }else{
            ele.textContent = ele.textContent.replace(tagSplit.reverse()[0], Word);        
        }

    });


    
}

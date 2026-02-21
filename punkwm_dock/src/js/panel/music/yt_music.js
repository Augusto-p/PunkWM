async function YT_Music_Search(q) {

    const body = {
        "query": q,
        "context": {
            "client": {
                "visitorData": getVisitorData(),
                "rolloutToken": getRolloutToken(),
                "clientName": "WEB_REMIX",
                "clientVersion": "1.20260209.03.00"
            }
        },
        "inlineSettingStatus": "INLINE_SETTING_STATUS_ON",
        "params": "EgWKAQIIAWoSEAkQBBADEBAQBRAVEAoQDhAR"
    };

    let response = await fetch("https://music.youtube.com/youtubei/v1/search?prettyPrint=false", { method: "POST", body: JSON.stringify(body), redirect: "follow" });
    if (response.status) {
        let Songs = []
        let data = await response.json()
        let songs = data["contents"].tabbedSearchResultsRenderer.tabs[0].tabRenderer.content.sectionListRenderer.contents[0].musicShelfRenderer.contents;
        songs.forEach(song => {
            let cover = song.musicResponsiveListItemRenderer.thumbnail.musicThumbnailRenderer.thumbnail.thumbnails[1].url
            let id = song.musicResponsiveListItemRenderer.playlistItemData.videoId
            let title = song.musicResponsiveListItemRenderer.flexColumns[0].musicResponsiveListItemFlexColumnRenderer.text.runs[0].text;
            let texts = song.musicResponsiveListItemRenderer.flexColumns[1].musicResponsiveListItemFlexColumnRenderer.text.runs;
            let points = getPoints(texts);
            let artist = TextJoin(texts.slice(0, points[0]), " ");
            let album = TextJoin(texts.slice(points[0] + 1, points[1]), " ");
            let duration = TextJoin(texts.slice(points[1] + 1, points[2]), " ");
            Songs.push({
                "id": id,
                "title": title,
                "artist": artist,
                "cover": cover,
                "duration": duration,
                "album": album,
                "mode": "YT-Music"
            });
        });
        load_Songs(Songs);
    }


}

async function YT_Music_Quick() {
    const body = {
        "context": {
            "client": {
                "hl": "es-419",
                "gl": "UY",
                "remoteHost": "2800:a4:6f8:dd00:c5f9:7363:c432:b6b9",
                "deviceMake": "",
                "deviceModel": "",
                "visitorData": "Cgt5U204SC1kbG9QUSj-_OfMBjIKCgJVWRIEGgAgVQ%3D%3D",
                "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36,gzip(gfe)",
                "clientName": "WEB_REMIX",
                "clientVersion": "1.20260209.03.00",
                "osName": "Windows",
                "osVersion": "10.0",
                "originalUrl": "https://music.youtube.com/",
                "screenPixelDensity": 1,
                "platform": "DESKTOP",
                "clientFormFactor": "UNKNOWN_FORM_FACTOR",
                "configInfo": {
                    "appInstallData": "CP7858wGELfq_hIQwY_QHBDrzIATEMbGzxwQgc3OHBCU_rAFENr3zhwQ4uvQHBDM364FEL22rgUQ3NTQHBC9mbAFEOvm0BwQ2NfQHBDwnc8cEMj3zxwQu9nOHBCJsM4cEJS20BwQh6zOHBCxyYATEMn3rwUQ9quwBRCu1s8cEO_b0BwQvKTQHBCmttAcEMGq0BwQi_fPHBCV988cEJmNsQUQqp3PHBDy6tAcENPu0BwQvoqwBRDZxNAcELjkzhwQlIPQHBCe0LAFEMfo0BwQ3rzOHBCIh7AFEPG00BwQ0MKAExC56tAcEJfd0BwQ0e7QHBD8ss4cEPbW0BwQ-s7QHBCM6c8cENPhrwUQ28WAExDszdAcEPjo0BwQxuvQHCpEQ0FNU0xSVXUtWnEtRExpVUVwUUNuQTdNQmJiUDhBc3l2MV9wMVFVRHpmOEZ5N2NHLXlhMjVBYmQ4d1BzN3dRZEJ3PT0wAA%3D%3D",
                    "coldConfigData": "CP7858wGGjJBSVdMQ1gzcVFHdmNJelZMZjFucFJna2twX3BpTjl2azhKZnFsWGhxVXkyVXd3ZllkQSIyQUlXTENYMXBtSHJVWi1sVFN5eGVJanROc2xJVkNVazJadlQ3aEoyYlB6cG81OTFTMlE%3D",
                    "coldHashData": "CP7858wGEhM4MzcyMjg4Nzg1MDY2MDg0NzkyGP7858wGMjJBSVdMQ1gzcVFHdmNJelZMZjFucFJna2twX3BpTjl2azhKZnFsWGhxVXkyVXd3ZllkQToyQUlXTENYMXBtSHJVWi1sVFN5eGVJanROc2xJVkNVazJadlQ3aEoyYlB6cG81OTFTMlE%3D",
                    "hotHashData": "CP7858wGEhQxNTE2MTM5OTQ1Mjg5MDMyOTY2OBj-_OfMBjIyQUlXTENYM3FRR3ZjSXpWTGYxbnBSZ2trcF9waU45dms4SmZxbFhocVV5MlV3d2ZZZEE6MkFJV0xDWDFwbUhyVVotbFRTeXhlSWp0TnNsSVZDVWsyWnZUN2hKMmJQenBvNTkxUzJR"
                },
                "screenDensityFloat": 1.25,
                "userInterfaceTheme": "USER_INTERFACE_THEME_DARK",
                "timeZone": "America/Montevideo",
                "browserName": "Chrome",
                "browserVersion": "144.0.0.0",
                "acceptHeader": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
                "deviceExperimentId": "ChxOell3T1RNNU1qZzJNamc1TVRFek9UWXpOdz09EP7858wGGP7858wG",
                "rolloutToken": "CMXW8szXzqjJwwEQ68nmuurokgMYxarxuurokgM%3D",
                "screenWidthPoints": 640,
                "screenHeightPoints": 480,
                "utcOffsetMinutes": -180,
                "musicAppInfo": {
                    "pwaInstallabilityStatus": "PWA_INSTALLABILITY_STATUS_UNKNOWN",
                    "webDisplayMode": "WEB_DISPLAY_MODE_BROWSER",
                    "storeDigitalGoodsApiSupportStatus": {
                        "playStoreDigitalGoodsApiSupportStatus": "DIGITAL_GOODS_API_SUPPORT_STATUS_UNSUPPORTED"
                    }
                }
            },
            "user": {
                "lockedSafetyMode": false
            },
            "request": {
                "useSsl": true,
                "internalExperimentFlags": [],
                "consistencyTokenJars": []
            },
            "adSignalsInfo": {
                "params": [
                    {
                        "key": "dt",
                        "value": "1771699838872"
                    },
                    {
                        "key": "flash",
                        "value": "0"
                    },
                    {
                        "key": "frm",
                        "value": "0"
                    },
                    {
                        "key": "u_tz",
                        "value": "-180"
                    },
                    {
                        "key": "u_his",
                        "value": "4"
                    },
                    {
                        "key": "u_h",
                        "value": "960"
                    },
                    {
                        "key": "u_w",
                        "value": "1536"
                    },
                    {
                        "key": "u_ah",
                        "value": "912"
                    },
                    {
                        "key": "u_aw",
                        "value": "1536"
                    },
                    {
                        "key": "u_cd",
                        "value": "24"
                    },
                    {
                        "key": "bc",
                        "value": "31"
                    },
                    {
                        "key": "bih",
                        "value": "480"
                    },
                    {
                        "key": "biw",
                        "value": "625"
                    },
                    {
                        "key": "brdim",
                        "value": "84,107,84,107,1536,0,640,481,640,480"
                    },
                    {
                        "key": "vis",
                        "value": "1"
                    },
                    {
                        "key": "wgl",
                        "value": "true"
                    },
                    {
                        "key": "ca_type",
                        "value": "image"
                    }
                ]
            }
        },
        "browseId": "FEmusic_home"
    }


fetch("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false", );














    let response = await fetch("https://music.youtube.com/youtubei/v1/browse?prettyPrint=false", {
  "headers": {
    "accept": "*/*",
    "accept-language": "es,es-ES;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
    "authorization": "SAPISIDHASH 1771700861_6cd272c0a9a8254fa3a230da9e08deee9919e980_u SAPISID1PHASH 1771700861_6cd272c0a9a8254fa3a230da9e08deee9919e980_u SAPISID3PHASH 1771700861_6cd272c0a9a8254fa3a230da9e08deee9919e980_u",
    "content-type": "application/json",
    "priority": "u=1, i",
    "sec-ch-ua": "\"Not(A:Brand\";v=\"8\", \"Chromium\";v=\"144\", \"Microsoft Edge WebView2\";v=\"144\", \"Google Chrome\";v=\"144\"",
    "sec-ch-ua-arch": "\"x86\"",
    "sec-ch-ua-bitness": "\"64\"",
    "sec-ch-ua-form-factors": "\"Desktop\"",
    "sec-ch-ua-full-version": "\"144.0.7559.133\"",
    "sec-ch-ua-full-version-list": "\"Not(A:Brand\";v=\"8.0.0.0\", \"Chromium\";v=\"144.0.7559.133\", \"Microsoft Edge WebView2\";v=\"144.0.3719.115\", \"Google Chrome\";v=\"144.0.7559.133\"",
    "sec-ch-ua-mobile": "?0",
    "sec-ch-ua-model": "\"\"",
    "sec-ch-ua-platform": "\"Windows\"",
    "sec-ch-ua-platform-version": "\"19.0.0\"",
    "sec-ch-ua-wow64": "?0",
    "sec-fetch-dest": "empty",
    "sec-fetch-mode": "same-origin",
    "sec-fetch-site": "same-origin",
    "x-browser-channel": "stable",
    "x-browser-copyright": "Copyright 2025 Google LLC. All rights reserved.",
    "x-browser-validation": "5sIVVtVmIdhoPXzr4AHI3aD5P60=",
    "x-browser-year": "1969",
    "x-goog-authuser": "0",
    "x-goog-visitor-id": "Cgt5U204SC1kbG9QUSj-_OfMBjIKCgJVWRIEGgAgVQ%3D%3D",
    "x-origin": "https://music.youtube.com",
    "x-youtube-bootstrap-logged-in": "true",
    "x-youtube-client-name": "67",
    "x-youtube-client-version": "1.20260209.03.00"
  },
  "referrer": "https://music.youtube.com/channel/UCgQna2EqpzqzfBjlSmzT72w",
  "body": JSON.stringify(body),
  "method": "POST",
  "mode": "cors",
  "credentials": "include"
});
    if (response.status) {
        let data = await response.json()
        console.log(data.contents.singleColumnBrowseResultsRenderer.tabs[0].tabRenderer.content.sectionListRenderer.contents[0].musicCarouselShelfRenderer.contents[0].musicResponsiveListItemRenderer.flexColumns[0].musicResponsiveListItemFlexColumnRenderer.text.runs[0].text);

    }



}

function TextJoin(elements, separador) {
    if (elements.length == 0) {
        return "";
    } else if (elements.length == 1) {
        return elements[0].text;
    } else {
        let text = []
        elements.forEach(e => {
            text.push(e.text);
        });
        return text.join(separador);
    }
}

function getPoints(texts) {
    points = []
    for (let index = 0; index < texts.length; index++) {
        const element = texts[index].text;
        if (element.includes("•")) {
            points.push(index);
        }

    }
    points.push(texts.length);
    return points

}
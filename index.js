"use strict";


let fileInput = document.querySelector("#image");
fileInput.addEventListener("change", onImageSelected);

function onImageSelected() {
    let curFiles = fileInput.files;

    if (curFiles.length === 0) {

    } else {
        for (const file of curFiles) {
            console.log(file.name, file.size);
            console.log(file);
            // console.log(file.arrayBuffer());
            file.arrayBuffer().then((buf)=> {
                console.log(buf.byteLength);
                const array = new Int8Array(buf);
                console.log(get_length(array));
                console.log(image_width(array));
                console.log(get_buffer(12));
                
                let xlsx = img2xlsx(array, 100, undefined);
                console.log(xlsx);
                globalThis.xlsx = xlsx;
                let blob = new Blob([xlsx], {type:"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"});
                let url = URL.createObjectURL(blob);
                document.querySelector("#xlsx").href=url;
            })
            // let buffer = new Int8Array(file.arrayBuffer());
            // console.log(buffer.length);
            // console.log(get_length(buffer));
        }
    }
}
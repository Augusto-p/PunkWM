function Load_stats(CPU, RAM, SSD, GPU) {
    stats_Content.style.setProperty("--stats_CPU", `${CPU}%`);
    stats_Content.style.setProperty("--stats_RAM", `${RAM}%`);
    stats_Content.style.setProperty("--stats_SSD", `${SSD}%`);
    stats_Content.style.setProperty("--stats_GPU", `${GPU}%`);
    
}
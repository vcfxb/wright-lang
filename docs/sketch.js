var x, y;
var xdir = 1;
var ydir = 1;
var t_size = 40;

function setup() {
    createCanvas(windowWidth-20, windowHeight-20);
    background(color(65));
    textSize(t_size);
    frameRate(160);
    x = 0;
    y = t_size;
}

function draw() {
    textSize(t_size);
    fill(70,70,250);
    background(color(65));
    text("Wright's website is still a work in progress.", x, y);
    x += xdir;
    y += ydir;
    if (x == width-int(textWidth("Wright's website is still a work in progress."))) {
        xdir = -1;
    }
    else if (x == 0) {
        xdir = 1;
    }
    if (y == height) {
        ydir = -1;
    }
    else if (y == t_size) {
        ydir = 1;
    }
    textSize(12);
    fill(0);
    text("Built using p5.js", 10, height - 5);
}
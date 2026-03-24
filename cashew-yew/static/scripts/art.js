// Art background animations - JS port of Vue canvas components
(function() {
  'use strict';

  var r180 = Math.PI;
  var r90 = Math.PI / 2;
  var r15 = Math.PI / 12;
  var color = '#88888825';
  var MIN_BRANCH = 30;
  var LEN = 6;

  function polar2cart(x, y, r, theta) {
    return [x + r * Math.cos(theta), y + r * Math.sin(theta)];
  }

  function initPlum(canvas) {
    var ctx = canvas.getContext('2d');
    var dpr = window.devicePixelRatio || 1;
    var width = window.innerWidth;
    var height = window.innerHeight;

    canvas.width = dpr * width;
    canvas.height = dpr * height;
    canvas.style.width = width + 'px';
    canvas.style.height = height + 'px';
    ctx.scale(dpr, dpr);

    var steps = [];
    var prevSteps = [];
    var stopped = false;
    var rafId = null;

    function step(x, y, rad, counter) {
      if (!counter) counter = { value: 0 };
      var length = Math.random() * LEN;
      counter.value += 1;

      var next = polar2cart(x, y, length, rad);
      var nx = next[0], ny = next[1];

      ctx.beginPath();
      ctx.moveTo(x, y);
      ctx.lineTo(nx, ny);
      ctx.stroke();

      var rad1 = rad + Math.random() * r15 * 1.2;
      var rad2 = rad - Math.random() * r15 * 1.2;

      if (nx < -100 || nx > width + 100 || ny < -100 || ny > height + 100) return;

      var rate = counter.value <= MIN_BRANCH ? 0.8 : 0.5;

      if (Math.random() < rate)
        steps.push(function() { step(nx, ny, rad1, counter); });
      if (Math.random() < rate)
        steps.push(function() { step(nx, ny, rad2, counter); });
    }

    var lastTime = performance.now();
    var interval = 1000 / 40;

    function frame() {
      if (stopped) return;
      rafId = requestAnimationFrame(frame);

      if (performance.now() - lastTime < interval) return;

      prevSteps = steps;
      steps = [];
      lastTime = performance.now();

      if (!prevSteps.length) {
        stopped = true;
        return;
      }

      prevSteps.forEach(function(fn) {
        if (Math.random() < 0.5) steps.push(fn);
        else fn();
      });
    }

    function randomMiddle() { return Math.random() * 0.6 + 0.2; }

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.lineWidth = 1;
    ctx.strokeStyle = color;
    prevSteps = [];
    steps = [
      function() { step(randomMiddle() * width, height + 5, -r90); },
      function() { step(-5, randomMiddle() * height, 0); },
      function() { step(width + 5, randomMiddle() * height, r180); },
      function() { step(randomMiddle() * width, -5, r90); },
      function() { step(randomMiddle() * width, height + 5, -r90); }
    ];
    if (width < 500) steps = steps.slice(0, 2);

    stopped = false;
    frame();
  }

  function initDots(canvas) {
    var ctx = canvas.getContext('2d');
    var dpr = window.devicePixelRatio || 1;
    var width = window.innerWidth;
    var height = window.innerHeight;

    canvas.width = dpr * width;
    canvas.height = dpr * height;
    canvas.style.width = width + 'px';
    canvas.style.height = height + 'px';
    ctx.scale(dpr, dpr);

    var SPACING = 15;
    var cols = Math.ceil(width / SPACING);
    var rows = Math.ceil(height / SPACING);
    var dots = [];

    for (var r = 0; r < rows; r++) {
      for (var c = 0; c < cols; c++) {
        dots.push({
          x: c * SPACING + SPACING / 2,
          y: r * SPACING + SPACING / 2,
          ox: c * SPACING + SPACING / 2,
          oy: r * SPACING + SPACING / 2
        });
      }
    }

    var mouseX = -1000, mouseY = -1000;
    canvas.style.pointerEvents = 'auto';
    canvas.addEventListener('mousemove', function(e) {
      mouseX = e.clientX;
      mouseY = e.clientY;
    });
    canvas.addEventListener('mouseleave', function() {
      mouseX = -1000;
      mouseY = -1000;
    });
    canvas.style.pointerEvents = 'none';

    // Simple noise-like displacement using sin
    var t = 0;
    function frame() {
      requestAnimationFrame(frame);
      t += 0.01;

      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = '#88888830';

      for (var i = 0; i < dots.length; i++) {
        var d = dots[i];
        var nx = d.ox + Math.sin(t + d.ox * 0.01) * 3;
        var ny = d.oy + Math.cos(t + d.oy * 0.01) * 3;
        ctx.beginPath();
        ctx.arc(nx, ny, 1, 0, Math.PI * 2);
        ctx.fill();
      }
    }

    frame();
  }

  function initDust(canvas) {
    var ctx = canvas.getContext('2d');
    var dpr = window.devicePixelRatio || 1;
    var width = window.innerWidth;
    var height = window.innerHeight;

    canvas.width = dpr * width;
    canvas.height = dpr * height;
    canvas.style.width = width + 'px';
    canvas.style.height = height + 'px';
    ctx.scale(dpr, dpr);

    var particles = [];
    var count = Math.floor((width * height) / 800);
    if (count > 500) count = 500;

    for (var i = 0; i < count; i++) {
      particles.push({
        x: Math.random() * width,
        y: Math.random() * height,
        size: Math.random() * 2 + 0.5,
        speedX: (Math.random() - 0.5) * 0.3,
        speedY: (Math.random() - 0.5) * 0.3,
        opacity: Math.random() * 0.3 + 0.1
      });
    }

    function frame() {
      requestAnimationFrame(frame);
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      for (var i = 0; i < particles.length; i++) {
        var p = particles[i];
        p.x += p.speedX;
        p.y += p.speedY;

        if (p.x < 0) p.x = width;
        if (p.x > width) p.x = 0;
        if (p.y < 0) p.y = height;
        if (p.y > height) p.y = 0;

        ctx.beginPath();
        ctx.arc(p.x, p.y, p.size, 0, Math.PI * 2);
        ctx.fillStyle = 'rgba(136, 136, 136, ' + p.opacity + ')';
        ctx.fill();
      }
    }

    frame();
  }

  window.initArt = function(type) {
    var canvas = document.getElementById('art-canvas');
    if (!canvas) return;

    switch (type) {
      case 'plum': initPlum(canvas); break;
      case 'dots': initDots(canvas); break;
      case 'dust': initDust(canvas); break;
      default: initPlum(canvas);
    }
  };
})();

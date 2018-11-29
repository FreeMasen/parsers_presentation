
{{#playpen lib.rs}}

<script>
(function() {
    var pp = getPlaypen();
    var mode = localStorage.getItem('presentation_mode');
    if (!pp) return;
    pp.classList.add('article-content');
    if (mode == 1) {
        pp.classList.add('not-presenting');
    } else if (mode == 0) {
        pp.classList.add('presenting');
    }

    function getPlaypen() {
        var cls = '.playpen';
        var pre = document.querySelectorAll(cls);
        if (!pre || pre.length < 1) return console.error('failed to find', cls);
        pre = pre[pre.length - 1];
        if (!pre || !pre.parentElement) return console.error('unknown structure of', cls);
        return pre.parentElement;
    }
})()
</script>
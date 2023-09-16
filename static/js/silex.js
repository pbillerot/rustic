$(document).ready(function () {
    var $isUsed = false;
    var $crud_view = $('#crud_view').val();

    var $sid = sessionStorage.getItem('sid');
    if (!$sid) {
        sessionStorage.setItem('sid', Date.now());
        $sid = sessionStorage.getItem('sid');
    } // endif form
    document.getElementById('sid').value = $sid;

    // clic sur TR ou CARD
    $('.crud-jquery-url').on('click', function (event) {
        // id de l'onglet de la session

        if ($isUsed) {
            event.preventDefault();
            $isUsed = false;
            return
        }
        // Mémo de la ligne sélectionnée d'une table dans un cookie
        if ($(this).prop("nodeName") == "TR") {
            var $node = $(this).closest("table");
            var $appid = $node.data("appid");
            var $tableid = $node.data("tableid");
            var $viewid = $node.data("viewid");
            var $anchorid = $appid + "-" + $tableid + "-" + $viewid
            Cookies.set($anchorid, $(this).data("url"))
            $(this).addClass("crud-list-selected");
        } else if ($(this).hasClass("card")) {
            var $anchorid = $("#crud_view").val();
            Cookies.set($anchorid, $(this).data("url"))
            $(this).addClass("crud-list-selected");
        }
        var target = $(event.target);
        if (target.hasClass("crud-jquery-action") || target.parent().hasClass("crud-jquery-action")) {
            // pour laisser la main à crud-jquery-action
            // Cas d'un button dans une card
            event.preventDefault();
            return
        }
        if (target.hasClass("crud-jquery-ajax") || target.parent().hasClass("crud-jquery-ajax")) {
            // pour laisser la main à crud-jquery-ajax
            // Cas d'un button dans une card
            event.preventDefault();
            return
        }
        if (target.hasClass("crud-jquery-button") || target.parent().hasClass("crud-jquery-button")) {
            // pour laisser la main à crud-jquery-button
            // Cas d'un button dans une card
            event.preventDefault();
            return
        }
        if (target.hasClass("crud-popup-image") || target.parent().hasClass("crud-popup-image")) {
            // pour laisser la main à crud-jquery-button
            // Cas d'un button dans une card
            event.preventDefault();
            return
        }

        var $url = $(this).data('url');
        window.location = $url;
        event.preventDefault();
    });

    // positionnement sur le dernier item sélectionné dans une table
    $('table').each(function (index) {
        if ($crud_view && $crud_view.length > 0) {
            // nous sommes dans une vue simple
            // et non pas dans une carte avec une vue
            // on passe
            return;
        }
        var $node = $(this);
        var $appid = $node.data("appid");
        var $tableid = $node.data("tableid");
        var $viewid = $node.data("viewid");
        var $anchorid = $appid + "-" + $tableid + "-" + $viewid

        if (Cookies.get($anchorid)) {
            // est-ce que l'ancre existe ?
            var $anchor = $('tr[data-url="' + Cookies.get($anchorid) + '"]');
            if ($anchor.length) {
                // oui, c'est super
                // recherche du container scrollable
                var $container = $anchor.closest('div');
                $container[0].scrollTo({
                    top: $anchor.position().top - 100,
                    left: 0,
                    behavior: 'smooth'
                });
                $anchor.addClass("crud-list-selected");
            }
        }
    });
    // positionnement sur la dernière carte sélectionnée
    // sélection sur le clic sur crud-jquery-url
    if ($crud_view && $crud_view.length > 0) {
        var $anchorid = $crud_view
        if (Cookies.get($anchorid)) {
            var $anchor = $('div[data-url="' + Cookies.get($anchorid) + '"]');
            if ($anchor.length) {
                // cas des vues list_card
                $('html, body').animate({
                    scrollTop: $anchor.offset().top - 200
                }, 1000);
                $anchor.removeClass("raised");
                $anchor.addClass("crud-list-selected");
            } else {
                $anchor = $('tr[data-url="' + Cookies.get($anchorid) + '"]');
                if ($anchor.length) {
                    // cas des vues list_table
                    $('html, body').animate({
                        scrollTop: $anchor.offset().top - 200
                    }, 1000);
                    $anchor.removeClass("raised");
                    $anchor.addClass("crud-list-selected");
                }
            }
        }
    };
    // Dropdown menu
    $('.ui.dropdown').dropdown();
});
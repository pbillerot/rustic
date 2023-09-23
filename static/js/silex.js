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

    // RECHERCHE dans une vue
    // Recherche via les filtres
    $('.crud-filter-go').on('click', function (event) {
        $('#formFilter', document).submit();
        event.preventDefault();
    });
    // Clear des filtres de la vue
    $('.crud-filter-clear').on('click', function (event) {
        $('#resetfilter').val("reset");
        $('#formFilter', document).submit();
        event.preventDefault();
    });

    // Recherche globale dans la barre
    $('.crud-search-active').on('click', function (event) {
        var $content = $(this).closest('.crud-search-div');
        $content.find('.crud-search').show();
        $content.find('.crud-search-active').hide();
        $content.find('.header').hide();
        $content.find('.meta').hide();
        $content.find('input').focus();
        event.preventDefault();
    });
    // Fermer la recherche
    $('.crud-search-close').on('click', function (event) {
        var $content = $(this).closest('.crud-search-div');
        $content.find('.crud-search-input').val('');
        $content.find('.crud-search').hide();
        $content.find('.crud-search-active').show();
        $content.find('.header').show();
        $content.find('.meta').show();
        if ($content.find('.crud-search-input-1').val().length == 0) {
            // abandon d'une recherche
            event.preventDefault();
            return;
        }
        $content.find('.crud-search-go').trigger('click');
        event.preventDefault();
    });
    // Validation par touche entrée
    $('.crud-search-input').on('keypress', function (e) {
        var $content = $(this).closest('.crud-search-div');
        if (e.which == 13) {
            $content.find('.crud-search-go').trigger('click');
        }
    });
    // Envoi de la recherche au serveur
    $('.crud-search-go').on('click', function (event) {
        var $content = $(this).closest('.crud-search-div');
        var $datas = {"search": $content.find('.crud-search-input').val().toLowerCase()};
        var $url = $content.find('.crud-search-input').data("url");
        var request =
            $.ajax({
                type: "POST",
                url: $url,
                data: JSON.stringify($datas),
                dataType: 'json',
                cache: false,
                contentType: "application/json; charset=utf-8",
            });
        request.done(function (response) {
            //Code à jouer en cas d'éxécution sans erreur
            console.log(response);
        });
        request.fail(function (response) {
            //Code à jouer en cas d'éxécution en erreur
            console.log(response);
        });
        request.always(function () {
            //Code à jouer après done OU fail dans tous les cas
            window.location.reload();
        });
        event.preventDefault();
    });
    // Boucle pour faire apparaître les champs de recherche avec une valeur
    $('.crud-search-input').each(function (index) {
        var $content = $(this).closest('.crud-search-div');
        if ($(this).val().length > 0) {
            // backup search dans crud-search-input-1 afin de traiter l'abandon d'une recherche
            $content.find('.crud-search-input-1').val($(this).val());
            $content.find('.crud-search-active').trigger('click');
        }
    });

    // TRI COLONNE DE LA TABLE
    $(document).on('click', '.crud-ajax-sort', function (event) {
        var $sortdirection = "ascending"
        if (!$(this).hasClass('sorted')) {
            $(this).closest('tr').find('.sorted').removeClass('sorted');
            $(this).closest('tr').find('.ascending').removeClass('ascending');
            $(this).closest('tr').find('.descending').removeClass('descending');
            $(this).addClass("sorted")
            $(this).addClass($sortdirection)
        } else {
            // on inverse le tri
            if ($(this).hasClass('ascending')) {
                $(this).closest('tr').find('.ascending').removeClass('ascending');
                $sortdirection = "descending"
                $(this).addClass($sortdirection)
            }
        }
        var $sortid = this.id.substring(4)

        var $datas = {"sortid": $sortid, "sortdirection": $sortdirection};
        var $node = $(this).closest('table');
        // $datas.append("sortid", $sortid);
        // $datas.append("sortdirection", $sortdirection);
        // $datas.append("_xsrf", $("#xsrf").val());
        var request =
            $.ajax({
                type: "POST",
                url: "/sort/" + $node.data("appid") + "/" + $node.data("tableid") + "/" + $node.data("viewid"),
                data: JSON.stringify($datas),
                dataType: 'json',
                cache: false,
                contentType: "application/json; charset=utf-8",
                processData: false,
            });
        request.done(function (response) {
            //Code à jouer en cas d'éxécution sans erreur
            console.log(response);
        });
        request.fail(function (response) {
            //Code à jouer en cas d'éxécution en erreur
            console.log(response);
        });
        request.always(function () {
            //Code à jouer après done OU fail dans tous les cas
            window.location.reload();
        });

        event.preventDefault();
    });
    // TRI retour au tri par défaut de la vue CLICK-DROIT
    $(".crud-unsort").on("press", function (event) {
        var $datas = new FormData();
        var $node = $(this).closest('table');
        $datas.append("sortid", "");
        $datas.append("sortdirection", "");
        // $datas.append("_xsrf", $("#xsrf").val());
        var request =
            $.ajax({
                type: "POST",
                url: "/sort/" + $node.data("appid") + "/" + $node.data("tableid") + "/" + $node.data("viewid"),
                data: $datas,
                dataType: 'json',
                cache: false,
                contentType: false,
                processData: false,
            });
        request.done(function (response) {
            //Code à jouer en cas d'éxécution sans erreur
            console.log(response);
        });
        request.fail(function (response) {
            //Code à jouer en cas d'éxécution en erreur
            console.log(response);
        });
        request.always(function () {
            //Code à jouer après done OU fail dans tous les cas
            window.location.reload();
        });

        event.preventDefault();
    });

    // ACTION DEMANDE CONFIRMATION
    $('.crud-jquery-action').on('click', function (event) {
        var $url = $(this).data('url');
        if ($(this).data('confirm') == true) {
            $('#crud-action').html($(this).html());
            $('#crud-modal-confirm')
                .modal({
                    closable: false,
                    onDeny: function () {
                        return true;
                    },
                    onApprove: function () {
                        $('#beeForm').attr('action', $url);
                        $('#beeForm', document).submit();
                    }
                }).modal('show');
        } else {
            // Sans demande de confirmation
            $('#beeForm').attr('action', $url);
            $('#beeForm', document).submit()
        }
        event.preventDefault();
    });

    // query sql en ajax
    $('.crud-jquery-ajax').on('click', function (event) {
        var $datas = new FormData();
        var $url = $(this).data('url');
        $.ajax({
            type: "POST",
            url: $url,
            data: $datas,
            dataType: 'json',
            cache: false,
            contentType: false,
            processData: false,
        })
            .done(function (data) {
                // code en cas de succès
            })
            .fail(function (error) {
                // code en cas d'échec
            })
            .always(function () {
                // code systématique
                window.location.reload(true);
            });
        event.preventDefault();
    });


    // Exécute en ajax une requête SQL sur le serveur et remplit les champs reçus du formulaire courant
    $('.crud-ajax-sql').on('click', function (event) {
        var $datas = new FormData();
        var $url = $(this).data('url');
        // ajout de données variables à la requête POST
        var $dataset = $(this).data();
        for (var d in $dataset) {
            if (d == "url") continue;
            var val = $("#" + $dataset[d]).val()
            $datas.append(d, val);
        }
        // var xsrf = $("#xsrf").val();
        // $datas.append("_xsrf", xsrf);
        $.ajax({
            type: "POST",
            url: $url,
            data: $datas,
            dataType: 'json',
            cache: false,
            contentType: false,
            processData: false,
        })
            //Ce code sera exécuté en cas de succès - La réponse du serveur est passée à done()
            //On peut par exemple convertir cette réponse en chaine JSON et insérer cette chaine dans un div id="res"
            .done(function (data) {
                if (data.Response != "ok") {
                    $.toast({
                        message: data.Message,
                        class: 'error',
                        className: {
                            toast: 'ui message'
                        },
                        position: 'bottom center',
                        minDisplayTime: 1500
                    });
                } else {
                    // mise à jour des rubriques trouvées dans la table
                    for (var rub in data.Dataset) {
                        if ($('#' + rub).is("select")) {
                            $('#' + rub).dropdown('set selected', data.Dataset[rub]);
                        } else {
                            $('#' + rub).val(data.Dataset[rub])
                        }

                    }
                    $.toast({
                        message: data.Message,
                        class: 'success',
                        className: {
                            toast: 'ui message'
                        },
                        position: 'bottom center',
                        minDisplayTime: 1500
                    });
                }
                //$("div#res").append(mes);
            })
            //Ce code sera exécuté en cas d'échec - L'erreur est passée à fail()
            //On peut afficher les informations relatives à la requête et à l'erreur
            .fail(function (error) {
                let mes = JSON.stringify(error);
                alert(mes)
                $.toast({
                    message: "La requête s'est terminée en échec. Infos : " + JSON.stringify(error),
                    class: 'error',
                    className: {
                        toast: 'ui message'
                    },
                    position: 'bottom center',
                    minDisplayTime: 1500
                });
            })
        //Ce code sera exécuté que la requête soit un succès ou un échec
        // .always(function () {
        //   setTimeout(() => { window.location.reload(true) }, 1500);
        // });
        event.preventDefault();
    });

    // UI SEMANTIC
    $('.ui.dropdown').dropdown();
    $('.message .close')
        .on('click', function () {
            $(this)
                .closest('.message')
                .transition('fade')
                ;
        }
        );
    $('.hide')
        .on('click', function () {
            $(this)
                .closest('.message')
                .transition('fade')
                ;
        }
        );

    // Toaster
    $('#toaster')
        .toast({
            class: $('#toaster').data('color'),
            position: 'bottom center',
            message: $('#toaster').val()
        });

});
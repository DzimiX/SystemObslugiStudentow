userdata.id_uprawnien = 2; // nadpisanie widoku

let id_uzytkownik = {
    "id_uzytkownik" : userdata.id
}

$.ajax({
    url: '/api/auth/aktywne',
    type: "POST",
    data: JSON.stringify(id_uzytkownik),
    contentType: "application/json; charset=UTF-8",
    success: function (data) {            
        if (data.status != 200){
            if (data.status == 401){
                wyloguj();
            } else {
                document.location.href = "/konto_wylaczone.html";
            }
        }
    }
});

$.get("navbar.html", function(data){
    $("#navbar").html(data);
   
    widok(userdata.id_uprawnien)

    function widok(id_uprawnien){
        
        if(id_uprawnien == 1){ // UŻYTKOWNIK
            $("#dropdown-wiadomosci").show();
            $("#menu-ogloszenia").hide();
            $("#dropdown-uzytkownicy").hide();
            $("#menu-zarzadzanie-zapisy").hide();
            $("#menu-rekrutacja").show();
            $("#menu-zapisy").show();
            $("#menu-zarzadzanie-kursy").hide();
            $("#dropdown-kursy").hide();
            $("#menu-kursy-prowadzacy").hide();
            $("#menu-indeks").hide();
            $("#dropdown-sprawy").show();
            $("#dropdown-sprawy-pracownik").hide();
            $("#dropdown-sprawyz").hide();           
        }

        if(id_uprawnien == 2){ // STUDENT
            $("#dropdown-wiadomosci").show();
            $("#menu-ogloszenia").hide();
            $("#dropdown-uzytkownicy").hide();
            $("#menu-zarzadzanie-zapisy").hide();
            $("#menu-rekrutacja").hide();
            $("#menu-zapisy").show();
            $("#menu-zarzadzanie-kursy").hide();
            $("#dropdown-kursy").show();
            $("#menu-kursy-prowadzacy").hide();
            $("#menu-indeks").show();
            $("#dropdown-sprawy").show();
            $("#dropdown-sprawy-pracownik").hide();
            $("#dropdown-sprawyz").hide();  
        }

        if(id_uprawnien == 3){ // PROWADZĄCY
            $("#dropdown-wiadomosci").show();
            $("#menu-ogloszenia").hide();
            $("#dropdown-uzytkownicy").hide();
            $("#menu-zarzadzanie-zapisy").hide();
            $("#menu-rekrutacja").hide();
            $("#menu-zapisy").hide();
            $("#menu-zarzadzanie-kursy").hide();
            $("#dropdown-kursy").hide();
            $("#menu-kursy-prowadzacy").show();
            $("#menu-indeks").hide();
            $("#dropdown-sprawy").hide();
            $("#dropdown-sprawy-pracownik").show();
            $("#dropdown-sprawyz").hide();
        }

        if(id_uprawnien == 4){ // PRACOWNIK
            $("#dropdown-wiadomosci").show();
            $("#menu-ogloszenia").show();
            $("#dropdown-uzytkownicy").hide();
            $("#menu-zarzadzanie-zapisy").show();
            $("#menu-rekrutacja").hide();
            $("#menu-zapisy").hide();
            $("#menu-zarzadzanie-kursy").show();
            $("#dropdown-kursy").hide();
            $("#menu-kursy-prowadzacy").hide();
            $("#menu-indeks").hide();
            $("#dropdown-sprawy").hide();
            $("#dropdown-sprawy-pracownik").show();
            $("#dropdown-sprawyz").show();
        }

        if(id_uprawnien == 5){
            $("#dropdown-wiadomosci").show();
            $("#menu-ogloszenia").show();
            $("#dropdown-uzytkownicy").show();
            $("#menu-zarzadzanie-zapisy").show();
            $("#menu-rekrutacja").hide();
            $("#menu-zapisy").hide();
            $("#menu-zarzadzanie-kursy").show();
            $("#dropdown-kursy").hide();
            $("#menu-kursy-prowadzacy").hide();
            $("#menu-indeks").hide();
            $("#dropdown-sprawy").hide();
            $("#dropdown-sprawy-pracownik").show();
            $("#dropdown-sprawyz").show();
        }
    }

    $("#wyloguj_navbar").click(function() {
        deleteAllCookies();
        document.location.href = "/logowanie.html";
    });

    switch (userdata.id_uprawnien) {
        case 6 :
            document.getElementById("auth-ranga").innerHTML = "DEBUG";
            break;
        case 5 :
            document.getElementById("auth-ranga").innerHTML = "Administrator";
            break;
        case 4 :
            document.getElementById("auth-ranga").innerHTML = "Pracownik";
            break;
        case 3 :
            document.getElementById("auth-ranga").innerHTML = "Prowadzący";
            break;
        case 2 :
            document.getElementById("auth-ranga").innerHTML = "Student";
            break;
        case 1 :
            document.getElementById("auth-ranga").innerHTML = "Użytkownik";
            break;
        default:
            document.getElementById("auth-ranga").innerHTML = "";
    }

    let uzytkownik_id = {
        "id" : userdata.id
    }

    $.ajax({
        url: '/api/uzytkownik',
        type: "POST",
        data: JSON.stringify(uzytkownik_id),
        contentType: "application/json; charset=UTF-8",
        success: function (data) {            
            if (data.status != 200){
                if (data.status == 401){
                    wyloguj();
                }
            } else if (data.status == 200){
                document.getElementById("auth-imie").innerHTML = data.result['imie'];
                document.getElementById("auth-nazwisko").innerHTML = data.result['nazwisko'];
            }
        }
    });
});
userdata.id_uprawnien = 6; // nadpisanie widoku
widok(userdata.id_uprawnien)

function widok(id_uprawnien){
    
    if(id_uprawnien == 1 || id_uprawnien == 2 || id_uprawnien == 3){
        $("#menu-ogloszenia").hide();
        $("#dropdown-uzytkownicy").hide();
        $("#menu-zarzadzanie-zapisy").hide();
        $("#dropdown-kursyz").hide();
        $("#dropdown-sprawyz").hide();
    }

    if(id_uprawnien == 1){
        $("#dropdown-kursy").hide();
        $("#menu-indeks").hide();
    }

    if(id_uprawnien == 3 || id_uprawnien == 4 || id_uprawnien == 5){
        $("#menu-indeks").hide();
    }

    if(id_uprawnien == 4){
        $("#dropdown-uzytkownicy").hide();
        $("#menu-zapisy").hide();
        $("#dropdown-kursy").hide();
        $("#dropdown-sprawy").hide();
    }

    if(id_uprawnien == 4){
        $("#dropdown-uzytkownicy").hide();
        $("#menu-zapisy").hide();
        $("#dropdown-kursy").hide();
        $("#dropdown-sprawy").hide();
    }

    if(id_uprawnien == 5){
        $("#menu-zapisy").hide();
        $("#dropdown-kursy").hide();
        $("#dropdown-sprawy").hide();
    }
}

function wyloguj(){
    deleteAllCookies();
    document.location.href = "/logowanie.html";
}

$("#wyloguj_navbar").click(function() {
    deleteAllCookies();
    document.location.href = "/logowanie.html";
});

let uzytkownik_id = {
    "id" : userdata.id
}

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

$.ajax({
    url: '/api/uzytkownik',
    type: "POST",
    data: JSON.stringify(uzytkownik_id),
    contentType: "application/json; charset=UTF-8",
    success: function (data) {
        console.log(data.status);
        console.log(data.result);
        
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
//https://jsfiddle.net/MwKpP/2/
function getCookie(c_name) {
    var c_value = document.cookie,
        c_start = c_value.indexOf(" " + c_name + "=");
    if (c_start == -1) c_start = c_value.indexOf(c_name + "=");
    if (c_start == -1) {
        c_value = null;
    } else {
        c_start = c_value.indexOf("=", c_start) + 1;
        var c_end = c_value.indexOf(";", c_start);
        if (c_end == -1) {
            c_end = c_value.length;
        }
        c_value = unescape(c_value.substring(c_start, c_end));
    }
    return c_value;
}

//https://www.w3schools.com/js/js_cookies.asp
function setCookie(c_name, value, exdays) {
    var exdate = new Date();
    exdate.setDate(exdate.getDate() + exdays);
    var c_value = escape(value) + ((exdays == null) ? "" : "; expires=" + exdate.toUTCString());
    document.cookie = c_name + "=" + c_value;
}

//https://stackoverflow.com/questions/179355/clearing-all-cookies-with-javascript
function deleteAllCookies() {
    var cookies = document.cookie.split(";");

    for (var i = 0; i < cookies.length; i++) {
        var cookie = cookies[i];
        var eqPos = cookie.indexOf("=");
        var name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
        document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
    }
}

//https://stackoverflow.com/questions/19491336/how-to-get-url-parameter-using-jquery-or-plain-javascript
var getUrlParameter = function getUrlParameter(sParam) {
    var sPageURL = window.location.search.substring(1),
        sURLVariables = sPageURL.split('&'),
        sParameterName,
        i;

    for (i = 0; i < sURLVariables.length; i++) {
        sParameterName = sURLVariables[i].split('=');

        if (sParameterName[0] === sParam) {
            return typeof sParameterName[1] === undefined ? true : decodeURIComponent(sParameterName[1]);
        }
    }
    return false;
};

function wyloguj(){
    deleteAllCookies();
    document.location.href = "/logowanie.html";
}

//https://www.w3schools.com/howto/howto_js_sort_table.asp
function sortTable(tbname,collumn,order) {
    var table, rows, switching, i, x, y, shouldSwitch;
    table = document.getElementById(tbname);
    switching = true;

    while (switching) {
        switching = false;
        rows = table.rows;
        for (i = 1; i < (rows.length - 1); i++) {
            shouldSwitch = false;
            if(order){
                x = rows[i+1].getElementsByTagName("TD")[collumn];
                y = rows[i].getElementsByTagName("TD")[collumn];
            } else {
                x = rows[i].getElementsByTagName("TD")[collumn];
                y = rows[i+1].getElementsByTagName("TD")[collumn];
            }
            
            if (x.innerHTML.toLowerCase() > y.innerHTML.toLowerCase()) {
                shouldSwitch = true;
                break;
            }
        }
        if (shouldSwitch) {
            rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
            switching = true;
        }
    }
}

function filterTable(tbname,collumn,variable) {
    var filter, table, tr, td, i, txtValue;
    filter = variable.toUpperCase();
    table = document.getElementById(tbname);
    tr = table.getElementsByTagName("tr");
  
    for (i = 1; i < tr.length; i++) {
      td = tr[i].getElementsByTagName("td")[collumn];
      if (td) {
        txtValue = td.textContent || td.innerText;
        if (txtValue.toUpperCase().indexOf(filter) > -1) {
          tr[i].style.display = "";
        } else {
          tr[i].style.display = "none";
        }
      }
    }
  }
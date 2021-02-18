
$error = "<pre style='font-size:24px;'>If you see this, make sure libapache-php is installed or
php-fpm is installed and php being passed from nginx to php-fpm</pre>";
$currentDir = $_SERVER['DOCUMENT_ROOT']
?>
<!doctype html>
<html lang="en">
  <header>
    <title>You should not see this</title>
  </header>
  <body>
    <p>
      This page should not be visible, please set your document root to the
      public html direction.
    </p>
    <h2>Apache Ex.</h2>
    <pre>
&lt;VirtualHost *:80&gt;
  ServerName showsite
  DocumentRoot <?= $currentDir ?>/public_html/
  &lt;Directory <?= $currentDir ?>/public_html/&gt;
    Require all granted
    Options +Indexes
  &lt;/Directory&gt;
&lt;/VirtualHost&gt;
    </pre>
    <h2>Nginx Ex.</h2>
    <pre>
server {
  listen 80;
  listen [::]:80;

  server_name showsite;

  root '<?= $currentDir ?>/publichtml/';
  index index.php;
  location / {
    tryfiles &uri $uri/ =404;
  }
  location ~ \.php$ {
    include snippets/fastcgi-php.conf;
    fastcgi_pass unix:/run/php/php7.4-fpm.sock;
  }
}
    </pre>
</html>

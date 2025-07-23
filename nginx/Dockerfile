# nginx/Dockerfile
FROM nginx:1.25.5-alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY ../dist /var/www

#RUN url=$(curl -sL https://api.github.com/repos/frederikbroth/cv-adventure/releases/latest \
#      | grep "browser_download_url" \
#      | grep ".zip" \
#      | cut -d '"' -f 4) && \
#      wget $url
#RUN unzip dist.zip
#RUN mv dist /var/www

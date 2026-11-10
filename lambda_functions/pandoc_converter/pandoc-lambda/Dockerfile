FROM public.ecr.aws/lambda/python:3.11

# Install system dependencies for WeasyPrint
RUN yum install -y \
    pango \
    pango-devel \
    gdk-pixbuf2 \
    cairo \
    cairo-devel \
    libffi-devel \
    wget \
    tar \
    gzip \
    && yum clean all

# Install Pandoc from official release
RUN wget -qO- https://github.com/jgm/pandoc/releases/download/3.8.3/pandoc-3.8.3-linux-amd64.tar.gz | \
    tar xvzf - --strip-components 1 -C /usr/local

# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Fix WeasyPrint/fontconfig cache permissions for Lambda
ENV FONTCONFIG_PATH=/tmp/fonts
ENV XDG_CACHE_HOME=/tmp/.cache

# Copy function code
COPY app.py ${LAMBDA_TASK_ROOT}
RUN chmod 644 ${LAMBDA_TASK_ROOT}/app.py

CMD ["app.handler"]

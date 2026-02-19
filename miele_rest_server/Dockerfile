ARG BUILD_FROM
FROM ${BUILD_FROM}

WORKDIR /app

RUN if command -v apk >/dev/null 2>&1; then \
      apk add --no-cache python3 py3-pip; \
    elif command -v apt-get >/dev/null 2>&1; then \
      apt-get update && apt-get install -y --no-install-recommends python3 python3-pip \n      && rm -rf /var/lib/apt/lists/*; \n    else \n      echo "No supported package manager found (apk/apt-get)." >&2; \n      exit 1; \n    fi \n    && pip3 install --break-system-packages --no-cache-dir \n      "https://github.com/hannesdelbeke/MieleRESTServer/archive/refs/heads/master.zip"

COPY . ./
RUN chmod a+x /app/run.sh

CMD ["/app/run.sh"]
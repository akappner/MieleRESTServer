FROM python:3.13-slim-bookworm

WORKDIR /app

COPY requirements.txt ./
RUN pip3 install --no-cache-dir -r requirements.txt

COPY . ./

CMD ["python3", "Server.py", "-b", "0.0.0.0", "-p", "5001", "-c", "/config/MieleRESTServer.config"]

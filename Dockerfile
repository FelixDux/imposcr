FROM konstin2/maturin as maturin
WORKDIR /io
COPY ./imposclib .
RUN /usr/bin/maturin build --interpreter python3.9 --release --strip

FROM python:3.9-slim-buster
COPY --from=maturin /io/target/wheels /imposclib

WORKDIR /imposc

COPY ./imposc .

RUN python -m pip install -r requirements.txt

RUN ls /imposclib/* |xargs python -m pip install 

ENV PYTHONPATH="/imposc"

EXPOSE 8000

CMD [ "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000" ]

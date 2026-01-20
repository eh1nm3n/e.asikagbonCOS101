--
-- PostgreSQL database dump
--

\restrict s618sW2aNbs8zLZJqS7a7Bj8nehtnhNnNf2nZ4r0gle9xqEfVbxYNTUQaTTINag

-- Dumped from database version 18.1
-- Dumped by pg_dump version 18.1

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size text NOT NULL,
    data_duration_days integer NOT NULL,
    data_price_naira real
);


ALTER TABLE public.dataplan OWNER TO postgres;

--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan (data_id, data_size, data_duration_days, data_price_naira) FROM stdin;
1	350 MB	2	200
2	1.8 GB	14	500
3	3.9 GB	30	1000
4	7.5 GB	30	1500
5	9.2 GB	30	2000
6	10.8 GB	30	2500
7	14 GB	30	3000
8	18 GB	30	4000
9	24 GB	30	5000
10	29.9 GB	30	8000
11	50 GB	30	10000
\.


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- PostgreSQL database dump complete
--

\unrestrict s618sW2aNbs8zLZJqS7a7Bj8nehtnhNnNf2nZ4r0gle9xqEfVbxYNTUQaTTINag


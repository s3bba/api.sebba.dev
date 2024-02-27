{{- define "domain.cert.name" -}}
{{- printf "%s-cert" .Values.service.domain | trunc 63 }}
{{- end }}